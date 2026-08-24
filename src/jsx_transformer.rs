use std::{matches, vec};

use crate::import_manager::{ImportManager, ImportName};
use swc_core::atoms::Wtf8Atom;
use swc_core::common::{util::take::Take, Mark, SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

fn null_expr() -> Expr {
    Expr::Lit(Lit::Null(Null::dummy()))
}

fn array_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    })
}

fn object_expr() -> Expr {
    Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![],
    })
}

fn str_expr(value: Wtf8Atom) -> Expr {
    Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value,
        raw: None,
    }))
}

fn prop_expr(expr: Expr) -> ExprOrSpread {
    ExprOrSpread {
        spread: None,
        expr: Box::new(expr),
    }
}

fn call_expr(callee: Expr, args: Vec<ExprOrSpread>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(callee)),
        args,
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

fn is_fn_component(ident: &Ident) -> bool {
    matches!(
        ident.sym.as_bytes().first(),
        Some(b'A'..=b'Z' | b'_' | b'$')
    )
}

fn convert_jsx_member_expr(jsx_memeber: JSXMemberExpr) -> Expr {
    let obj_expr = match jsx_memeber.obj {
        JSXObject::Ident(ident) => Expr::Ident(ident),
        JSXObject::JSXMemberExpr(member) => convert_jsx_member_expr(*member),
    };

    Expr::Member(MemberExpr {
        span: jsx_memeber.span,
        obj: Box::new(obj_expr),
        prop: MemberProp::Ident(jsx_memeber.prop),
    })
}

fn convert_jsx_namespaced_name(jsx_namespaced: JSXNamespacedName) -> Expr {
    let name_str: String = format!("{}:{}", jsx_namespaced.ns, jsx_namespaced.name);
    str_expr(name_str.into())
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
            JSXElementChild::JSXExprContainer(container) => match &container.expr {
                JSXExpr::JSXEmptyExpr(_) => None,
                JSXExpr::Expr(expr) => Some(Some(prop_expr(expr.as_ref().clone()))),
            },
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
    match &element.opening.name {
        JSXElementName::Ident(ident) => {
            if is_fn_component(ident) {
                call_expr(Expr::Ident(ident.clone()), vec![prop_expr(object_expr())])
            } else {
                let mut args = vec![
                    prop_expr(str_expr(ident.sym.clone().into())),
                    prop_expr(object_expr()),
                ];

                if !children.is_empty() {
                    args.push(prop_expr(children_expr(children)));
                }

                call_expr(Expr::Ident(imports.add(ImportName::Jsx)), args)
            }
        }
        JSXElementName::JSXMemberExpr(jsx_memeber) => call_expr(
            convert_jsx_member_expr(jsx_memeber.clone()),
            vec![prop_expr(object_expr())],
        ),
        JSXElementName::JSXNamespacedName(jsx_memeber) => call_expr(
            Expr::Ident(imports.add(ImportName::Jsx)),
            vec![
                prop_expr(convert_jsx_namespaced_name(jsx_memeber.clone())),
                prop_expr(object_expr()),
            ],
        ),
    }
}

pub struct JsxTransformer {
    pub imports: ImportManager,
}

impl JsxTransformer {
    pub fn new() -> Self {
        Self {
            imports: ImportManager::new(SyntaxContext::empty().apply_mark(Mark::new())),
        }
    }
}

impl VisitMut for JsxTransformer {
    fn visit_mut_module(&mut self, node: &mut Module) {
        node.visit_mut_children_with(self);
        self.imports.inject_into_module(node);
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
