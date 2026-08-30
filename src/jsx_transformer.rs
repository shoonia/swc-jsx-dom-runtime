use crate::builders::*;
use crate::collections::*;
use crate::consts::*;
use crate::import_manager::*;
use core::hint::unreachable_unchecked;
use std::vec;
use swc_core::common::{comments::Comments, errors::HANDLER, Span, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

fn non_lit_jsx_attr_val(attr: &JSXAttr) -> bool {
    let Some(value) = attr.value.as_ref() else {
        return false;
    };

    if let JSXAttrValue::Str(_) = value {
        return false;
    }

    if let JSXAttrValue::JSXExprContainer(container) = value {
        if let JSXExpr::Expr(expr) = &container.expr {
            if let Expr::Lit(_) = expr.as_ref() {
                return false;
            }
        }
    }

    true
}

fn convert_jsx_member(jsx_memeber: JSXMemberExpr) -> Expr {
    let obj_expr = match jsx_memeber.obj {
        JSXObject::Ident(ident) => Expr::Ident(ident),
        JSXObject::JSXMemberExpr(member) => convert_jsx_member(*member),
        _ => unsafe { unreachable_unchecked() },
    };

    Expr::Member(MemberExpr {
        span: jsx_memeber.span,
        obj: Box::new(obj_expr),
        prop: MemberProp::Ident(jsx_memeber.prop),
    })
}

fn convert_jsx_namespaced_name(jsx_namespaced: &JSXNamespacedName) -> String {
    let name = format!("{}:{}", jsx_namespaced.ns, jsx_namespaced.name);
    name
}

fn convert_jsx_container(container: &JSXExprContainer) -> Expr {
    match &container.expr {
        JSXExpr::Expr(expr) => expr.as_ref().clone(),
        _ => null_expr(),
    }
}

fn children_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    if elems.len() == 1 {
        match &elems[0] {
            Some(ExprOrSpread {
                spread: Some(_),
                expr: _,
            }) => array_expr(elems),
            Some(ExprOrSpread { spread: None, expr }) => expr.as_ref().clone(),
            None => null_expr(),
        }
    } else {
        array_expr(elems)
    }
}

#[derive(Clone, Copy)]
struct ParentScope {
    is_svg: bool,
    is_mathml: bool,
}

struct NodeScope {
    is_svg: bool,
    is_html: bool,
    is_mathml: bool,
    is_custom: bool,
}

pub struct JsxTransformer<C: Comments> {
    comments: Option<C>,
    imports: ImportManager,
    parent_scope: ParentScope,
}

impl<C: Comments> JsxTransformer<C> {
    pub fn new(comments: Option<C>) -> Self {
        Self {
            comments,
            imports: ImportManager::new(),
            parent_scope: ParentScope {
                is_svg: false,
                is_mathml: false,
            },
        }
    }

    fn add_pure_comment(&self, span: Span) {
        if let Some(comments) = self.comments.as_ref() {
            comments.add_pure_comment(span.lo);
        }
    }

    fn build_props(&mut self, attributes: &Vec<JSXAttrOrSpread>) -> Vec<PropOrSpread> {
        attributes
            .iter()
            .filter_map(|attr| match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let key = match &attr.name {
                        JSXAttrName::Ident(ident) => ident.sym.as_str(),
                        JSXAttrName::JSXNamespacedName(namespaced) => {
                            &convert_jsx_namespaced_name(namespaced)
                        }
                        _ => unsafe { unreachable_unchecked() },
                    };

                    Some(prop(prop_key(key), self.convert_jsx_attr_value(attr)))
                }
                JSXAttrOrSpread::SpreadElement(spread) => {
                    Some(PropOrSpread::Spread(SpreadElement {
                        dot3_token: spread.dot3_token,
                        expr: spread.expr.clone(),
                    }))
                }
                _ => None,
            })
            .collect()
    }

    fn transform_fragment(&mut self, fragment: &JSXFragment) -> Expr {
        let elems = self.build_children(&fragment.children);
        if elems.is_empty() {
            null_expr()
        } else {
            children_expr(elems)
        }
    }

    fn transform_element(&mut self, element: &JSXElement) -> Expr {
        let children = self.build_children(&element.children);
        let mut props = self.build_props(&element.opening.attrs);

        match &element.opening.name {
            JSXElementName::Ident(ident) => {
                if is_fn_component(ident) {
                    if !children.is_empty() {
                        props.push(prop(prop_ident(CHILDREN_KEY), children_expr(children)));
                    }

                    call_expr(
                        Expr::Ident(ident.clone()),
                        vec![prop_expr(object_expr(props))],
                    )
                } else {
                    let mut args = vec![
                        prop_expr(str_expr(ident.sym.clone().into())),
                        prop_expr(object_expr(props)),
                    ];

                    if !children.is_empty() {
                        args.push(prop_expr(children_expr(children)));
                    }

                    self.add_pure_comment(element.span);
                    call_expr(self.imports.add(ImportName::Jsx), args)
                }
            }
            JSXElementName::JSXMemberExpr(jsx_memeber) => {
                if !children.is_empty() {
                    props.push(prop(prop_ident(CHILDREN_KEY), children_expr(children)));
                }

                call_expr(
                    convert_jsx_member(jsx_memeber.clone()),
                    vec![prop_expr(object_expr(props))],
                )
            }
            JSXElementName::JSXNamespacedName(name) => {
                let mut args = vec![
                    prop_expr(str_expr(convert_jsx_namespaced_name(name).into())),
                    prop_expr(object_expr(props)),
                ];

                if !children.is_empty() {
                    args.push(prop_expr(children_expr(children)));
                }

                self.add_pure_comment(element.span);
                call_expr(self.imports.add(ImportName::Jsx), args)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn convert_jsx_attr_value(&mut self, attr: &JSXAttr) -> Expr {
        match &attr.value {
            Some(value) => match value {
                JSXAttrValue::Str(lit) => str_expr(lit.value.clone()),
                JSXAttrValue::JSXExprContainer(container) => convert_jsx_container(container),
                JSXAttrValue::JSXElement(element) => self.transform_element(element.as_ref()),
                JSXAttrValue::JSXFragment(fragment) => self.transform_fragment(fragment),
                _ => unsafe { unreachable_unchecked() },
            },
            None => bool_expr(true),
        }
    }

    fn build_children(&mut self, children: &Vec<JSXElementChild>) -> Vec<Option<ExprOrSpread>> {
        children
            .iter()
            .map(|child| match child {
                JSXElementChild::JSXExprContainer(container) => {
                    Some(prop_expr(convert_jsx_container(container)))
                }
                JSXElementChild::JSXSpreadChild(spread) => Some(ExprOrSpread {
                    spread: Some(spread.span),
                    expr: spread.expr.clone(),
                }),
                JSXElementChild::JSXText(text) => Some(prop_expr(str_expr(text.value.clone()))),
                JSXElementChild::JSXElement(element) => {
                    Some(prop_expr(self.transform_element(element.as_ref())))
                }
                JSXElementChild::JSXFragment(fragment) => {
                    Some(prop_expr(self.transform_fragment(fragment)))
                }
                _ => None,
            })
            .collect()
    }

    fn transform_jsx_element(&mut self, element: &mut JSXElement, scope: NodeScope) {
        let node = &mut element.opening;
        let mut remove_indexes = Vec::<usize>::new();
        let mut events = Vec::<PropOrSpread>::new();
        let mut compile_refs = Vec::<Expr>::new();
        let mut user_refs = Vec::<Expr>::new();
        let mut children_props = Vec::<JSXAttr>::new();
        let mut no_ns = true;

        for zip_attr in node.attrs.iter_mut().enumerate() {
            let (index, attr) = zip_attr;

            if let JSXAttrOrSpread::SpreadElement(spread) = attr {
                HANDLER.with(|handler| {
                    handler
                        .struct_fatal("\n\nSyntaxError: HTML, SVG, MathML or Custom Elements must not have spread attributes.\n")
                        .set_span(spread.span())
                        .emit();
                });
                continue;
            }

            if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                if let JSXAttrName::Ident(ident) = &attr.name {
                    let attr_name = ident.sym.as_str();

                    match attr_name {
                        REF_KEY => {
                            remove_indexes.push(index);
                            user_refs.push(self.convert_jsx_attr_value(attr));
                            continue;
                        }
                        CHILDREN_KEY => {
                            remove_indexes.push(index);
                            children_props.push(attr.clone());
                            continue;
                        }
                        "style" => {
                            if non_lit_jsx_attr_val(attr) {
                                remove_indexes.push(index);
                                compile_refs.push(set_utility(
                                    self.imports.add(ImportName::SetStyle),
                                    self.convert_jsx_attr_value(attr),
                                ));
                            }
                            continue;
                        }
                        "dataset" => {
                            remove_indexes.push(index);
                            compile_refs.push(set_utility(
                                self.imports.add(ImportName::SetDataset),
                                self.convert_jsx_attr_value(attr),
                            ));
                            continue;
                        }
                        "attributes" => {
                            remove_indexes.push(index);
                            compile_refs.push(set_utility(
                                self.imports.add(ImportName::SetAttributes),
                                self.convert_jsx_attr_value(attr),
                            ));
                            continue;
                        }
                        NS_KEY => {
                            no_ns = false;
                            continue;
                        }
                        _ => {}
                    }

                    if scope.is_custom {
                        continue;
                    }

                    if let Some(a) = html_dom_attribute(attr_name) {
                        attr.name = jsx_attr_name(a);
                        continue;
                    }

                    if scope.is_svg {
                        if let Some(a) = svg_dom_attribute(attr_name) {
                            attr.name = jsx_attr_name(a);
                            continue;
                        }
                    }

                    let name = attr_name.to_lowercase();

                    if scope.is_html && name != attr_name {
                        attr.name = jsx_attr_name(&name);
                    }

                    if is_bool_attr(&name) {
                        if attr.value.is_none() {
                            attr.value = jsx_attr_val_str("");
                        }
                        continue;
                    }

                    if is_enumerated_attr(&name) {
                        if attr.value.is_none() {
                            attr.value = jsx_attr_val_str("true");
                        } else if let Some(JSXAttrValue::JSXExprContainer(container)) = &attr.value
                        {
                            if let JSXExpr::Expr(expr) = &container.expr {
                                if let Expr::Lit(Lit::Bool(val)) = expr.as_ref() {
                                    attr.value = jsx_attr_val_str(&val.value.to_string());
                                }
                            }
                        }
                        continue;
                    }

                    if name.starts_with("on") {
                        remove_indexes.push(index);
                        compile_refs.push(prop_assignment_expr(
                            &name,
                            self.convert_jsx_attr_value(attr),
                        ));
                        continue;
                    }
                } else if let JSXAttrName::JSXNamespacedName(namespaced) = &attr.name {
                    let ns = namespaced.ns.sym.as_str();

                    match ns {
                        "on" => {
                            let ev_name = namespaced.name.sym.as_str();
                            let lc_name = &ev_name.to_lowercase();
                            let name = if event_types(lc_name) {
                                prop_ident(lc_name)
                            } else {
                                prop_key(ev_name)
                            };

                            remove_indexes.push(index);
                            events.push(prop(name, self.convert_jsx_attr_value(attr)));
                            continue;
                        }
                        "attr" => {
                            let value = self.convert_jsx_attr_value(attr);
                            let name = namespaced.name.sym.as_str();
                            let ref_expr = match value {
                                Expr::Lit(_) | Expr::Tpl(_) | Expr::Array(_) | Expr::Object(_) => {
                                    set_attr_call_expr(name, value)
                                }
                                _ => signalish_attr(
                                    self.imports.add(ImportName::SetSignalish),
                                    name,
                                    value,
                                ),
                            };

                            remove_indexes.push(index);
                            compile_refs.push(ref_expr);
                            continue;
                        }
                        "prop" => {
                            let value = self.convert_jsx_attr_value(attr);
                            let name = namespaced.name.sym.as_str();
                            let ref_expr = match value {
                                Expr::Lit(_) | Expr::Tpl(_) | Expr::Array(_) | Expr::Object(_) => {
                                    prop_assignment_expr(name, value)
                                }
                                _ => signalish_prop(
                                    self.imports.add(ImportName::SetSignalish),
                                    name,
                                    value,
                                ),
                            };

                            remove_indexes.push(index);
                            compile_refs.push(ref_expr);
                            continue;
                        }
                        _ => {}
                    }

                    if scope.is_custom {
                        continue;
                    }

                    if ns == "xlink" && namespaced.name.sym == "href" {
                        attr.name = JSXAttrName::from(namespaced.name.clone());
                    }
                }
            }
        }

        for index in remove_indexes.into_iter().rev() {
            node.attrs.remove(index);
        }

        if !events.is_empty() {
            node.attrs.push(jsx_attr(EVENT_KEY, object_expr(events)));
        }

        let refs = if compile_refs.is_empty() {
            user_refs
        } else {
            user_refs.push(create_ref_cb(compile_refs));
            user_refs
        };

        if !refs.is_empty() {
            node.attrs.push(jsx_attr(
                REF_KEY,
                match refs.len() {
                    1 => refs.into_iter().next().unwrap(),
                    _ => array_expr(refs.into_iter().map(|e| Some(prop_expr(e))).collect()),
                },
            ));
        }

        if !children_props.is_empty() && element.children.is_empty() {
            let last_children = children_props.pop().unwrap();
            let value = self.convert_jsx_attr_value(&last_children);

            element
                .children
                .push(JSXElementChild::JSXExprContainer(JSXExprContainer {
                    span: last_children.span,
                    expr: JSXExpr::Expr(Box::new(value)),
                }));
        }

        if no_ns {
            if scope.is_svg || self.parent_scope.is_svg {
                node.attrs
                    .push(jsx_attr(NS_KEY, self.imports.add(ImportName::SvgNs)));
            } else if scope.is_mathml || self.parent_scope.is_mathml {
                node.attrs
                    .push(jsx_attr(NS_KEY, self.imports.add(ImportName::MathmlNs)));
            }
        }
    }
}

impl<C: Comments> VisitMut for JsxTransformer<C> {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        self.imports.inject_into_module(module);
    }

    fn visit_mut_jsx_element(&mut self, element: &mut JSXElement) {
        let tag_name = match &element.opening.name {
            JSXElementName::Ident(ident) if !is_fn_component(ident) => ident.sym.as_str(),
            _ => return,
        };

        let is_html = is_html_tag(tag_name);
        let is_svg = is_svg_tag(tag_name);
        let is_mathml = is_mathml_tag(tag_name);
        let is_standard = is_html || is_svg || is_mathml;
        let is_custom = !is_standard && tag_name.contains('-');

        if !(is_standard || is_custom) {
            return;
        }

        let prev_parent_scope = self.parent_scope;

        self.parent_scope = ParentScope {
            is_svg: is_svg || self.parent_scope.is_svg,
            is_mathml: is_mathml || self.parent_scope.is_mathml,
        };
        self.transform_jsx_element(
            element,
            NodeScope {
                is_svg,
                is_html,
                is_mathml,
                is_custom,
            },
        );

        element.children.visit_mut_with(self);
        self.parent_scope = prev_parent_scope;
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        let expr = match node {
            Expr::JSXFragment(fragment) => self.transform_fragment(fragment),
            Expr::JSXElement(element) => self.transform_element(element.as_ref()),
            _ => return,
        };

        *node = expr;
    }
}
