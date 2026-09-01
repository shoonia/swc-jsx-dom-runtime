use crate::consts::*;
use swc_core::common::{Span, SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::*;
use swc_core::ecma::utils::is_valid_prop_ident;

#[inline(always)]
pub fn null_expr() -> Expr {
    Null { span: DUMMY_SP }.into()
}

#[inline(always)]
pub fn bool_expr(value: bool) -> Expr {
    value.into()
}

#[inline(always)]
pub fn array_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    ArrayLit {
        span: DUMMY_SP,
        elems,
    }
    .into()
}

#[inline(always)]
pub fn object_expr(props: Vec<PropOrSpread>) -> Expr {
    ObjectLit {
        span: DUMMY_SP,
        props,
    }
    .into()
}

#[inline(always)]
pub fn prop_expr(expr: Expr) -> ExprOrSpread {
    expr.into()
}

#[inline]
pub fn call_expr_with_span(callee: Expr, args: Vec<ExprOrSpread>, span: Span) -> Expr {
    CallExpr {
        span,
        callee: Box::new(callee).into(),
        args,
        type_args: None,
        ctxt: SyntaxContext::empty(),
    }
    .into()
}

#[inline(always)]
pub fn call_expr(callee: Expr, args: Vec<ExprOrSpread>) -> Expr {
    call_expr_with_span(callee, args, DUMMY_SP)
}

#[inline(always)]
pub fn prop_ident(key: &str) -> PropName {
    IdentName::from(key).into()
}

#[inline]
pub fn prop_key(key: &str) -> PropName {
    if is_valid_prop_ident(key) {
        IdentName::from(key).into()
    } else {
        Str::from(key).into()
    }
}

#[inline]
pub fn prop(key: PropName, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(
        KeyValueProp {
            key,
            value: Box::new(value),
        }
        .into(),
    ))
}

#[inline(always)]
pub fn jsx_attr_val_str(value: &str) -> Option<JSXAttrValue> {
    Some(Str::from(value).into())
}

#[inline]
pub fn jsx_attr(name: &str, value: Expr) -> JSXAttrOrSpread {
    JSXAttr {
        span: DUMMY_SP,
        name: IdentName::from(name).into(),
        value: Some(
            JSXExprContainer {
                span: DUMMY_SP,
                expr: Box::new(value).into(),
            }
            .into(),
        ),
    }
    .into()
}

#[inline(always)]
fn arrow_fn_param(name: &str) -> Pat {
    Ident::from(name).into()
}

#[inline]
fn arrow_fn_expr(param: Pat, body: ArrowFunctionBody) -> Expr {
    ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![param],
        body: Box::new(body),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    }
    .into()
}

pub fn create_ref_cb(refs: Vec<Expr>) -> Expr {
    let body = match refs.len() {
        1 => ArrowFunctionBody::Expr(Box::new(refs.into_iter().next().unwrap())),
        _ => ArrowFunctionBody::FunctionBody(FunctionBody {
            span: DUMMY_SP,
            stmts: refs
                .into_iter()
                .map(|expr| {
                    ExprStmt {
                        span: DUMMY_SP,
                        expr: expr.into(),
                    }
                    .into()
                })
                .collect(),
        }),
    };

    arrow_fn_expr(arrow_fn_param(REF_PARAM_KEY), body)
}

#[inline]
fn ref_param() -> Expr {
    Ident::from(REF_PARAM_KEY).into()
}

#[inline]
pub fn set_attr_call_expr(key: &str, value: Expr) -> Expr {
    call_expr(
        MemberExpr {
            span: DUMMY_SP,
            obj: ref_param().into(),
            prop: IdentName::from("setAttribute").into(),
        }
        .into(),
        vec![prop_expr(key.into()), prop_expr(value)],
    )
}

pub fn prop_assignment_expr(key: &str, value: Expr) -> Expr {
    AssignExpr {
        span: DUMMY_SP,
        op: AssignOp::Assign,
        left: MemberExpr {
            span: DUMMY_SP,
            obj: ref_param().into(),
            prop: if is_valid_prop_ident(key) {
                IdentName::from(key).into()
            } else {
                ComputedPropName {
                    span: DUMMY_SP,
                    expr: key.to_string().into(),
                }
                .into()
            },
        }
        .into(),
        right: value.into(),
    }
    .into()
}

pub fn set_utility(callee: Expr, value: Expr) -> Expr {
    call_expr(callee, vec![prop_expr(ref_param()), prop_expr(value)])
}

pub fn signalish_prop(callee: Expr, name: &str, value: Expr) -> Expr {
    let cb = arrow_fn_expr(
        arrow_fn_param(SIGNAL_PARAM_KEY),
        ArrowFunctionBody::Expr(Box::new(prop_assignment_expr(
            name,
            Ident::from(SIGNAL_PARAM_KEY).into(),
        ))),
    );

    call_expr(callee, vec![prop_expr(value), prop_expr(cb)])
}

pub fn signalish_attr(callee: Expr, name: &str, value: Expr) -> Expr {
    let cb = arrow_fn_expr(
        arrow_fn_param(SIGNAL_PARAM_KEY),
        ArrowFunctionBody::Expr(Box::new(set_attr_call_expr(
            name,
            Ident::from(SIGNAL_PARAM_KEY).into(),
        ))),
    );

    call_expr(callee, vec![prop_expr(value), prop_expr(cb)])
}
