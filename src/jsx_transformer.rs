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

fn fn_param(expr: Expr) -> ExprOrSpread {
    ExprOrSpread {
        spread: None,
        expr: Box::new(expr),
    }
}

fn call_expr(callee: Ident, args: Vec<ExprOrSpread>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(callee))),
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

fn build_children(children: &Vec<JSXElementChild>) -> Vec<Option<ExprOrSpread>> {
    children
        .iter()
        .filter_map(|child| match child {
            JSXElementChild::JSXExprContainer(container) => match &container.expr {
                JSXExpr::JSXEmptyExpr(_) => None,
                JSXExpr::Expr(expr) => Some(Some(ExprOrSpread {
                    spread: None,
                    expr: expr.clone(),
                })),
            },
            JSXElementChild::JSXSpreadChild(spread) => Some(Some(ExprOrSpread {
                spread: Some(spread.span),
                expr: spread.expr.clone(),
            })),
            JSXElementChild::JSXText(text) => Some(Some(ExprOrSpread {
                spread: None,
                expr: Box::new(str_expr(text.value.clone())),
            })),
            _ => None,
        })
        .collect()
}

fn transform_fragment(fragment: &JSXFragment) -> Expr {
    let elems: Vec<Option<ExprOrSpread>> = build_children(&fragment.children);

    if elems.is_empty() {
        null_expr()
    } else if elems.len() == 1 {
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

fn transform_element(element: &JSXElement, imports: &mut ImportManager) -> Expr {
    match &element.opening.name {
        JSXElementName::Ident(ident) => {
            if is_fn_component(ident) {
                call_expr(ident.clone(), vec![fn_param(object_expr())])
            } else {
                call_expr(
                    imports.add(ImportName::Jsx),
                    vec![
                        fn_param(str_expr(ident.sym.clone().into())),
                        fn_param(object_expr()),
                    ],
                )
            }
        }
        _ => null_expr(),
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
            Expr::JSXFragment(fragment) => transform_fragment(fragment),
            Expr::JSXElement(element) => transform_element(element, &mut self.imports),
            _ => return,
        };

        *node = expr;
    }
}
