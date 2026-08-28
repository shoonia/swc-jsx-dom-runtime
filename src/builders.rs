use crate::consts::*;
use swc_core::atoms::Wtf8Atom;
use swc_core::common::{util::take::Take, SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::*;
use swc_core::ecma::utils::is_valid_prop_ident;

#[inline]
pub fn null_expr() -> Expr {
    Expr::Lit(Lit::Null(Null::dummy()))
}

#[inline]
pub fn bool_expr(value: bool) -> Expr {
    Expr::Lit(Lit::Bool(Bool::from(value)))
}

#[inline]
pub fn array_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    })
}

#[inline]
pub fn object_expr(props: Vec<PropOrSpread>) -> Expr {
    Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    })
}

#[inline]
pub fn str_expr(value: Wtf8Atom) -> Expr {
    Expr::Lit(Lit::Str(Str::from(value)))
}

#[inline]
pub fn prop_expr(expr: Expr) -> ExprOrSpread {
    ExprOrSpread::from(Box::new(expr))
}

#[inline]
pub fn call_expr(callee: Expr, args: Vec<ExprOrSpread>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(callee)),
        args,
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

#[inline]
pub fn prop_ident(key: &str) -> PropName {
    PropName::Ident(IdentName::from(key))
}

#[inline]
pub fn prop_key(key: &str) -> PropName {
    if is_valid_prop_ident(key) {
        prop_ident(key)
    } else {
        PropName::Str(Str::from(key))
    }
}

#[inline]
pub fn prop(key: PropName, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key,
        value: Box::new(value),
    })))
}

#[inline]
pub fn jsx_attr_val_str(value: &str) -> Option<JSXAttrValue> {
    Some(JSXAttrValue::Str(Str::from(value)))
}

#[inline]
pub fn jsx_attr_name(name: &str) -> JSXAttrName {
    JSXAttrName::Ident(IdentName::from(name))
}

#[inline]
pub fn jsx_attr(name: &str, value: Expr) -> JSXAttrOrSpread {
    JSXAttrOrSpread::JSXAttr(JSXAttr {
        span: DUMMY_SP,
        name: jsx_attr_name(name),
        value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(Box::new(value)),
        })),
    })
}

#[inline]
fn arrow_fn_param(name: &str) -> Pat {
    Pat::Ident(BindingIdent::from(Ident::from(name)))
}

#[inline]
fn arrow_fn_expr(param: Pat, body: ArrowFunctionBody) -> Expr {
    Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![param],
        body: Box::new(body),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    })
}

pub fn create_ref_cb(refs: Vec<Expr>) -> Expr {
    let body = if refs.len() == 1 {
        ArrowFunctionBody::Expr(Box::new(refs[0].clone()))
    } else {
        ArrowFunctionBody::FunctionBody(FunctionBody {
            span: DUMMY_SP,
            stmts: refs
                .into_iter()
                .map(|expr| {
                    Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: Box::new(expr),
                    })
                })
                .collect(),
        })
    };

    arrow_fn_expr(arrow_fn_param(REF_PARAM_KEY), body)
}

#[inline]
fn ref_param() -> Expr {
    Expr::Ident(Ident::from(REF_PARAM_KEY))
}

#[inline]
pub fn set_attr_call_expr(key: &str, value: Expr) -> Expr {
    call_expr(
        Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(ref_param()),
            prop: MemberProp::Ident(IdentName::from("setAttribute")),
        }),
        vec![prop_expr(str_expr(key.into())), prop_expr(value)],
    )
}

pub fn prop_assignment_expr(key: &str, value: Expr) -> Expr {
    Expr::Assign(AssignExpr {
        span: DUMMY_SP,
        op: AssignOp::Assign,
        left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(ref_param()),
            prop: if is_valid_prop_ident(key) {
                MemberProp::Ident(IdentName::from(key))
            } else {
                MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(str_expr(key.into())),
                })
            },
        })),
        right: Box::new(value),
    })
}

pub fn set_utility(callee: Expr, value: Expr) -> Expr {
    call_expr(callee, vec![prop_expr(ref_param()), prop_expr(value)])
}

pub fn signalish_prop(callee: Expr, name: &str, value: Expr) -> Expr {
    let cb = arrow_fn_expr(
        arrow_fn_param(SIGNAL_PARAM_KEY),
        ArrowFunctionBody::Expr(Box::new(prop_assignment_expr(
            name,
            Expr::Ident(Ident::from(SIGNAL_PARAM_KEY)),
        ))),
    );

    call_expr(callee, vec![prop_expr(value), prop_expr(cb)])
}

pub fn signalish_attr(callee: Expr, name: &str, value: Expr) -> Expr {
    let cb = arrow_fn_expr(
        arrow_fn_param(SIGNAL_PARAM_KEY),
        ArrowFunctionBody::Expr(Box::new(set_attr_call_expr(
            name,
            Expr::Ident(Ident::from(SIGNAL_PARAM_KEY)),
        ))),
    );

    call_expr(callee, vec![prop_expr(value), prop_expr(cb)])
}
