use crate::builders::*;
use crate::collections::*;
use crate::import_manager::*;
use std::vec;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

const CHILDREN_KEY: &str = "children";
const NS_KEY: &str = "_";

fn convert_jsx_member(jsx_memeber: JSXMemberExpr) -> Expr {
    let obj_expr = match jsx_memeber.obj {
        JSXObject::Ident(ident) => Expr::Ident(ident),
        JSXObject::JSXMemberExpr(member) => convert_jsx_member(*member),
    };

    Expr::Member(MemberExpr {
        span: jsx_memeber.span,
        obj: Box::new(obj_expr),
        prop: MemberProp::Ident(jsx_memeber.prop),
    })
}

fn convert_jsx_namespaced_name(jsx_namespaced: JSXNamespacedName) -> String {
    format!("{}:{}", jsx_namespaced.ns, jsx_namespaced.name) as String
}

fn convert_jsx_container(container: &JSXExprContainer) -> Expr {
    match &container.expr {
        JSXExpr::JSXEmptyExpr(_) => null_expr(),
        JSXExpr::Expr(expr) => expr.as_ref().clone(),
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
            JSXElementChild::JSXElement(element) => {
                Some(Some(prop_expr(transform_element(element, imports))))
            }
            JSXElementChild::JSXFragment(fragment) => {
                Some(Some(prop_expr(transform_fragment(fragment, imports))))
            }
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
                        &convert_jsx_namespaced_name(namespaced.clone())
                    }
                };

                let value = match &attr.value {
                    Some(value) => match value {
                        JSXAttrValue::Str(lit) => str_expr(lit.value.clone()),
                        JSXAttrValue::JSXExprContainer(container) => {
                            convert_jsx_container(container)
                        }
                        JSXAttrValue::JSXElement(element) => transform_element(element, imports),
                        JSXAttrValue::JSXFragment(fragment) => {
                            transform_fragment(fragment, imports)
                        }
                    },
                    None => bool_expr(true),
                };

                Some(prop(prop_key(key), value))
            }
            JSXAttrOrSpread::SpreadElement(spread) => Some(PropOrSpread::Spread(SpreadElement {
                dot3_token: spread.dot3_token,
                expr: spread.expr.clone(),
            })),
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
                if is_svg_tag(&ident.sym) {
                    props.push(prop(prop_ident(NS_KEY), imports.add(ImportName::SvgNs)));
                } else if is_mathml_tag(&ident.sym) {
                    props.push(prop(prop_ident(NS_KEY), imports.add(ImportName::MathmlNs)));
                }

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
                prop_expr(str_expr(convert_jsx_namespaced_name(name.clone()).into())),
                prop_expr(object_expr(props)),
            ];

            if !children.is_empty() {
                args.push(prop_expr(children_expr(children)));
            }
            call_expr(imports.add(ImportName::Jsx), args)
        }
    }
}

pub struct JsxTransformer {
    pub imports: ImportManager,
}

impl JsxTransformer {
    pub fn new() -> Self {
        Self {
            imports: ImportManager::new(),
        }
    }
}

impl VisitMut for JsxTransformer {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        self.imports.inject_into_module(module);
    }

    fn visit_mut_jsx_opening_element(&mut self, node: &mut JSXOpeningElement) {
        match &node.name {
            JSXElementName::JSXMemberExpr(_) | JSXElementName::JSXNamespacedName(_) => return,
            JSXElementName::Ident(ident) => {
                if is_fn_component(ident) {
                    return;
                }
            }
        };

        for attr in node.attrs.iter_mut() {
            if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                if let JSXAttrName::Ident(ident) = &mut attr.name {
                    ident.sym = ident.sym.to_lowercase().into();
                }
            }
        }
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        let expr = match node {
            Expr::JSXFragment(fragment) => transform_fragment(fragment, &mut self.imports),
            Expr::JSXElement(element) => transform_element(element, &mut self.imports),
            _ => return,
        };

        *node = expr;
    }
}
