use swc_core::common::util::take::Take;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::visit::VisitMutWith;

fn null_expr() -> Expr {
    Expr::Lit(Lit::Null(Null::dummy()))
}

fn array_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    })
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
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: text.span,
                    value: text.value.clone(),
                    raw: None,
                }))),
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
            Some(ExprOrSpread { spread: None, expr }) => *expr.clone(),
            None => null_expr(),
        }
    } else {
        array_expr(elems)
    }
}

pub struct JsxTransformer;

impl VisitMut for JsxTransformer {
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        let expr = match node {
            Expr::JSXFragment(fragment) => transform_fragment(fragment),
            _ => return,
        };

        *node = expr;
    }
}
