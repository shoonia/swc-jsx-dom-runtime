use crate::builders::array_expr;
use swc_core::ecma::ast::{BinExpr, BinaryOp, Expr, ExprOrSpread, JSXAttr, JSXAttrValue, JSXExpr};

fn is_bin_lit(expr: &Expr) -> bool {
    if let Expr::Bin(BinExpr {
        op: BinaryOp::Add,
        left,
        ..
    }) = expr
    {
        return left.is_lit() || left.is_tpl();
    }

    false
}

pub fn is_non_lit_style(attr: &JSXAttr) -> bool {
    let Some(value) = &attr.value else {
        return false;
    };

    if let JSXAttrValue::Str(_) = value {
        return false;
    }

    if let JSXAttrValue::JSXExprContainer(container) = value {
        if let JSXExpr::Expr(expr) = &container.expr {
            if expr.is_lit() || expr.is_tpl() || is_bin_lit(expr) {
                return false;
            }
        }
    }

    true
}

#[inline]
pub fn is_non_signalish_value(expr: &Expr) -> bool {
    expr.is_lit() || expr.is_tpl() || expr.is_array() || expr.is_object() || is_bin_lit(expr)
}

pub fn children_expr(elems: Vec<ExprOrSpread>) -> Expr {
    let mut acc = Vec::with_capacity(elems.len());
    for elem in elems {
        flatten_child(elem, &mut acc);
    }

    if acc.len() == 1 {
        let elem = acc.pop().unwrap();
        if elem.spread.is_none() {
            *elem.expr
        } else {
            array_expr(vec![Some(elem)])
        }
    } else {
        array_expr(acc.into_iter().map(Some).collect())
    }
}

fn flatten_child(elem: ExprOrSpread, acc: &mut Vec<ExprOrSpread>) {
    if let Expr::Array(array) = *elem.expr {
        for item in array.elems.into_iter().flatten() {
            flatten_child(item, acc);
        }
    } else {
        acc.push(elem);
    }
}

fn flatten_expr(elem: Expr, acc: &mut Vec<Expr>) {
    if let Expr::Array(array) = elem {
        for item in array.elems.into_iter().flatten() {
            flatten_expr(*item.expr, acc);
        }
    } else {
        acc.push(elem);
    }
}

#[inline]
pub fn flatten_expr_vec(elems: Vec<Expr>) -> Vec<Expr> {
    let mut acc = Vec::with_capacity(elems.len());
    for elem in elems {
        flatten_expr(elem, &mut acc);
    }
    acc
}
