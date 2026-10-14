use std::matches;

use crate::builders::array_expr;
use swc_core::ecma::ast::{BinExpr, BinaryOp, Expr, ExprOrSpread, JSXAttr, JSXAttrValue, JSXExpr};

fn is_primitive_value(expr: &Expr) -> bool {
    if let Expr::Bin(BinExpr { op, .. }) = expr {
        return matches!(
            op,
            // Operators that can return non-primitive types
            // | BinaryOp::LogicalAnd // &&  -> any
            // | BinaryOp::LogicalOr  // ||  -> any
            // | BinaryOp::NullishCoalescing // ??  -> any

            // Operators that return only primitive types
            BinaryOp::EqEq             // ==  -> boolean
            | BinaryOp::NotEq          // !=  -> boolean
            | BinaryOp::EqEqEq         // === -> boolean
            | BinaryOp::NotEqEq        // !== -> boolean
            | BinaryOp::Lt             // <   -> boolean
            | BinaryOp::LtEq           // <=  -> boolean
            | BinaryOp::Gt             // >   -> boolean
            | BinaryOp::GtEq           // >=  -> boolean
            | BinaryOp::LShift         // <<  -> number / bigint
            | BinaryOp::RShift         // >>  -> number / bigint
            | BinaryOp::ZeroFillRShift // >>> -> number
            | BinaryOp::Add            // +   -> string / number / bigint
            | BinaryOp::Sub            // -   -> number / bigint
            | BinaryOp::Mul            // *   -> number / bigint
            | BinaryOp::Div            // /   -> number / bigint
            | BinaryOp::Mod            // %   -> number / bigint
            | BinaryOp::BitOr          // |   -> number / bigint
            | BinaryOp::BitXor         // ^   -> number / bigint
            | BinaryOp::BitAnd         // &   -> number / bigint
            | BinaryOp::In             // in  -> boolean
            | BinaryOp::InstanceOf     // instanceof -> boolean
            | BinaryOp::Exp // **  -> number / bigint
        );
    }

    expr.is_lit() || expr.is_tpl() || expr.is_unary()
}

pub fn is_non_lit_style(attr: &JSXAttr) -> bool {
    let Some(value) = &attr.value else {
        return false;
    };

    if let JSXAttrValue::Str(_) = value {
        return false;
    }

    if let JSXAttrValue::JSXExprContainer(container) = value
        && let JSXExpr::Expr(expr) = &container.expr
        && is_primitive_value(expr)
    {
        return false;
    }

    true
}

#[inline]
pub fn is_non_signalish_value(expr: &Expr) -> bool {
    is_primitive_value(expr) || expr.is_array() || expr.is_object()
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
