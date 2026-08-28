use crate::builders::*;
use crate::collections::*;
use crate::consts::*;
use crate::import_manager::*;
use core::hint::unreachable_unchecked;
use std::vec;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

fn is_jsx_attr_val_lit(attr: &JSXAttr) -> bool {
    let Some(value) = attr.value.as_ref() else {
        return true;
    };

    if let JSXAttrValue::Str(_) = value {
        return true;
    }

    if let JSXAttrValue::JSXExprContainer(container) = value {
        if let JSXExpr::Expr(expr) = &container.expr {
            if let Expr::Lit(_) = expr.as_ref() {
                return true;
            }
        }
    }

    false
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
    format!("{}:{}", jsx_namespaced.ns, jsx_namespaced.name) as String
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

fn convert_jsx_attr_value(attr: &JSXAttr, imports: &mut ImportManager) -> Expr {
    match &attr.value {
        Some(value) => match value {
            JSXAttrValue::Str(lit) => str_expr(lit.value.clone()),
            JSXAttrValue::JSXExprContainer(container) => convert_jsx_container(container),
            JSXAttrValue::JSXElement(element) => transform_element(element.as_ref(), imports),
            JSXAttrValue::JSXFragment(fragment) => transform_fragment(fragment, imports),
            _ => unsafe { unreachable_unchecked() },
        },
        None => bool_expr(true),
    }
}

fn build_children(
    children: &Vec<JSXElementChild>,
    imports: &mut ImportManager,
) -> Vec<Option<ExprOrSpread>> {
    children
        .iter()
        .filter_map(|child| match child {
            JSXElementChild::JSXExprContainer(container) => {
                Some(Some(prop_expr(convert_jsx_container(container))))
            }
            JSXElementChild::JSXSpreadChild(spread) => Some(Some(ExprOrSpread {
                spread: Some(spread.span),
                expr: spread.expr.clone(),
            })),
            JSXElementChild::JSXText(text) => Some(Some(prop_expr(str_expr(text.value.clone())))),
            JSXElementChild::JSXElement(element) => Some(Some(prop_expr(transform_element(
                element.as_ref(),
                imports,
            )))),
            JSXElementChild::JSXFragment(fragment) => {
                Some(Some(prop_expr(transform_fragment(fragment, imports))))
            }
            _ => None,
        })
        .collect()
}

fn build_props(
    attributes: &Vec<JSXAttrOrSpread>,
    imports: &mut ImportManager,
) -> Vec<PropOrSpread> {
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

                Some(prop(prop_key(key), convert_jsx_attr_value(attr, imports)))
            }
            JSXAttrOrSpread::SpreadElement(spread) => Some(PropOrSpread::Spread(SpreadElement {
                dot3_token: spread.dot3_token,
                expr: spread.expr.clone(),
            })),
            _ => None,
        })
        .collect()
}

fn transform_fragment(fragment: &JSXFragment, imports: &mut ImportManager) -> Expr {
    let elems = build_children(&fragment.children, imports);

    if elems.is_empty() {
        null_expr()
    } else {
        children_expr(elems)
    }
}

fn transform_element(element: &JSXElement, imports: &mut ImportManager) -> Expr {
    let children = build_children(&element.children, imports);
    let mut props = build_props(&element.opening.attrs, imports);

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

                call_expr(imports.add(ImportName::Jsx), args)
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
            call_expr(imports.add(ImportName::Jsx), args)
        }
        _ => unsafe { unreachable_unchecked() },
    }
}

struct ParentNode {
    is_svg: bool,
    is_mathml: bool,
}

pub struct JsxTransformer {
    imports: ImportManager,
    parent_node: ParentNode,
}

impl JsxTransformer {
    pub fn new() -> Self {
        Self {
            imports: ImportManager::new(),
            parent_node: ParentNode {
                is_svg: false,
                is_mathml: false,
            },
        }
    }
}

impl VisitMut for JsxTransformer {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        self.imports.inject_into_module(module);
    }

    fn visit_mut_jsx_opening_element(&mut self, node: &mut JSXOpeningElement) {
        let tag_name = match &node.name {
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

        let mut remove_indexes = Vec::<usize>::new();
        let mut events = Vec::<PropOrSpread>::new();
        let mut refs = Vec::<Expr>::new();

        for zip_attr in node.attrs.iter_mut().enumerate() {
            let (index, attr) = zip_attr;

            if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                if let JSXAttrName::Ident(ident) = &attr.name {
                    let attr_name = ident.sym.as_str();

                    match attr_name {
                        "style" => {
                            if is_jsx_attr_val_lit(attr) {
                                continue;
                            }
                            remove_indexes.push(index);
                            refs.push(set_utility(
                                self.imports.add(ImportName::SetStyle),
                                convert_jsx_attr_value(attr, &mut self.imports),
                            ));
                            continue;
                        }
                        "dataset" => {
                            remove_indexes.push(index);
                            refs.push(set_utility(
                                self.imports.add(ImportName::SetDataset),
                                convert_jsx_attr_value(attr, &mut self.imports),
                            ));
                            continue;
                        }
                        "attributes" => {
                            remove_indexes.push(index);
                            refs.push(set_utility(
                                self.imports.add(ImportName::SetAttributes),
                                convert_jsx_attr_value(attr, &mut self.imports),
                            ));
                            continue;
                        }
                        _ => {}
                    }

                    if is_custom {
                        continue;
                    }

                    if let Some(a) = HTML_DOM_ATTRIBUTES.get(attr_name) {
                        attr.name = jsx_attr_name(a);
                        continue;
                    }

                    if is_svg {
                        if let Some(a) = SVG_DOM_ATTRIBUTES.get(attr_name) {
                            attr.name = jsx_attr_name(a);
                        }
                        continue;
                    }

                    let name = attr_name.to_lowercase();

                    if is_html {
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
                        refs.push(prop_assignment_expr(
                            &name,
                            convert_jsx_attr_value(attr, &mut self.imports),
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
                            events
                                .push(prop(name, convert_jsx_attr_value(attr, &mut self.imports)));
                            continue;
                        }
                        "attr" => {
                            remove_indexes.push(index);
                            let value = convert_jsx_attr_value(attr, &mut self.imports);
                            let name = namespaced.name.sym.as_str();

                            if let Expr::Lit(_) = value {
                                refs.push(set_attr_call_expr(name, value));
                            } else {
                                refs.push(signalish_attr(
                                    self.imports.add(ImportName::SetSignalish),
                                    name,
                                    value,
                                ))
                            }

                            continue;
                        }
                        "prop" => {
                            remove_indexes.push(index);
                            let value = convert_jsx_attr_value(attr, &mut self.imports);
                            let name = namespaced.name.sym.as_str();

                            if let Expr::Lit(_) = value {
                                refs.push(prop_assignment_expr(name, value));
                            } else {
                                refs.push(signalish_prop(
                                    self.imports.add(ImportName::SetSignalish),
                                    name,
                                    value,
                                ))
                            }

                            continue;
                        }
                        _ => {}
                    }

                    if is_custom {
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

        if !refs.is_empty() {
            node.attrs.push(jsx_attr(REF_KEY, create_ref_cb(refs)));
        }

        if is_svg || self.parent_node.is_svg {
            node.attrs
                .push(jsx_attr(NS_KEY, self.imports.add(ImportName::SvgNs)));
        } else if is_mathml || self.parent_node.is_mathml {
            node.attrs
                .push(jsx_attr(NS_KEY, self.imports.add(ImportName::MathmlNs)));
        }

        self.parent_node = ParentNode { is_svg, is_mathml };
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        let expr = match node {
            Expr::JSXFragment(fragment) => transform_fragment(fragment, &mut self.imports),
            Expr::JSXElement(element) => transform_element(element.as_ref(), &mut self.imports),
            _ => return,
        };

        *node = expr;
    }
}
