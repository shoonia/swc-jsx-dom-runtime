use crate::builders::*;
use crate::collections::*;
use crate::consts::*;
use crate::import_manager::*;
use crate::jsx_text_to_str::{jsx_text_to_str_with_raw, transform_jsx_attr_str};
use core::hint::unreachable_unchecked;
use std::{iter, vec};
use swc_core::common::{comments::Comments, errors::HANDLER, Spanned};
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

fn convert_jsx_member(memeber: JSXMemberExpr) -> Expr {
    let obj_expr: Expr = match memeber.obj {
        JSXObject::Ident(ident) => ident.into(),
        JSXObject::JSXMemberExpr(member) => convert_jsx_member(*member),
        _ => unsafe { unreachable_unchecked() },
    };

    MemberExpr {
        span: memeber.span,
        obj: obj_expr.into(),
        prop: memeber.prop.into(),
    }
    .into()
}

fn convert_jsx_namespaced_name(jsx_namespaced: &JSXNamespacedName) -> String {
    format!("{}:{}", jsx_namespaced.ns, jsx_namespaced.name)
}

fn children_expr(elems: Vec<ExprOrSpread>) -> Expr {
    if elems.len() == 1 {
        match &elems[0] {
            ExprOrSpread {
                spread: Some(_),
                expr: _,
            } => array_expr(elems.into_iter().map(Some).collect()),
            ExprOrSpread { spread: None, expr } => expr.as_ref().clone(),
            _ => unsafe { unreachable_unchecked() },
        }
    } else {
        array_expr(elems.into_iter().map(Some).collect())
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
    comments: C,
    imports: ImportManager,
    parent_scope: ParentScope,
}

impl<C: Comments> JsxTransformer<C> {
    pub fn new(comments: C) -> Self {
        Self {
            comments,
            imports: ImportManager::new(),
            parent_scope: ParentScope {
                is_svg: false,
                is_mathml: false,
            },
        }
    }

    fn transform_expr(&mut self, expr: JSXExpr) -> Expr {
        match expr {
            JSXExpr::Expr(expr) => match expr.as_ref() {
                Expr::JSXElement(element) => self.transform_element(element.as_ref()),
                Expr::JSXFragment(fragment) => self.transform_fragment(fragment),
                Expr::JSXMember(memeber) => convert_jsx_member(memeber.clone()),
                _ => expr.as_ref().clone(),
            },
            _ => null_expr(),
        }
    }

    fn build_props(&mut self, attributes: &[JSXAttrOrSpread]) -> Vec<PropOrSpread> {
        attributes
            .iter()
            .filter_map(|attr| match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let key = match &attr.name {
                        JSXAttrName::Ident(ident) => ident.sym.as_str(),
                        JSXAttrName::JSXNamespacedName(namespaced) => {
                            &convert_jsx_namespaced_name(namespaced)
                        }
                        _ => None?,
                    };

                    Some(prop(prop_key(key), self.convert_jsx_attr_value(attr)))
                }
                JSXAttrOrSpread::SpreadElement(spread) => Some(
                    SpreadElement {
                        dot3_token: spread.dot3_token,
                        expr: spread.expr.clone(),
                    }
                    .into(),
                ),
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

                    call_expr(ident.clone().into(), vec![prop_expr(object_expr(props))])
                } else {
                    let mut args = vec![
                        prop_expr(ident.sym.clone().into()),
                        prop_expr(object_expr(props)),
                    ];

                    if !children.is_empty() {
                        args.push(prop_expr(children_expr(children)));
                    }

                    self.comments.add_pure_comment(element.span.lo);
                    call_expr_with_span(self.imports.add(ImportName::Jsx), args, element.span)
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
                    prop_expr(convert_jsx_namespaced_name(name).into()),
                    prop_expr(object_expr(props)),
                ];

                if !children.is_empty() {
                    args.push(prop_expr(children_expr(children)));
                }

                self.comments.add_pure_comment(element.span.lo);
                call_expr_with_span(self.imports.add(ImportName::Jsx), args, element.span)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn convert_jsx_attr_value(&mut self, attr: &JSXAttr) -> Expr {
        match &attr.value {
            Some(value) => match value {
                JSXAttrValue::Str(lit) => {
                    let value = lit.value.as_str().unwrap_or_default();
                    transform_jsx_attr_str(value).into()
                }
                JSXAttrValue::JSXExprContainer(cntr) => self.transform_expr(cntr.expr.clone()),
                JSXAttrValue::JSXElement(element) => self.transform_element(element.as_ref()),
                JSXAttrValue::JSXFragment(fragment) => self.transform_fragment(fragment),
                _ => unsafe { unreachable_unchecked() },
            },
            None => bool_expr(true),
        }
    }

    fn build_children(&mut self, children: &[JSXElementChild]) -> Vec<ExprOrSpread> {
        children
            .iter()
            .filter_map(|child| match child {
                JSXElementChild::JSXExprContainer(container) => match container.expr {
                    JSXExpr::JSXEmptyExpr(_) => None,
                    _ => Some(prop_expr(self.transform_expr(container.expr.clone()))),
                },
                JSXElementChild::JSXSpreadChild(spread) => Some(ExprOrSpread {
                    spread: Some(spread.span),
                    expr: spread.expr.clone(),
                }),
                JSXElementChild::JSXText(text) => {
                    let value = jsx_text_to_str_with_raw(&text.value, &text.raw);

                    if value.is_empty() {
                        None
                    } else {
                        Some(prop_expr(value.into()))
                    }
                }

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

        for (index, attr) in node.attrs.iter_mut().enumerate() {
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
                if let JSXAttrName::Ident(ident) = &mut attr.name {
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
                        ident.sym = a.into();
                        continue;
                    }

                    if scope.is_svg {
                        if let Some(a) = svg_dom_attribute(attr_name) {
                            ident.sym = a.into();
                            continue;
                        }
                    }

                    let name = attr_name.to_lowercase();

                    if scope.is_html && name != attr_name {
                        ident.sym = name.clone().into();
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
                            if name == "ondoubleclick" {
                                "ondblclick"
                            } else {
                                &name
                            },
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
            iter::once(create_ref_cb(compile_refs))
                .chain(user_refs)
                .collect()
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
            let last_children = children_props.last().unwrap();
            let value = self.convert_jsx_attr_value(&last_children);

            element.children.push(
                JSXExprContainer {
                    span: last_children.span,
                    expr: Box::new(value).into(),
                }
                .into(),
            );
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
