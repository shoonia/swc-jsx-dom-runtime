use swc_core::atoms::Wtf8Atom;
use swc_core::common::{util::take::Take, SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::*;
use swc_core::ecma::utils::is_valid_prop_ident;

#[inline(always)]
pub fn null_expr() -> Expr {
    Expr::Lit(Lit::Null(Null::dummy()))
}

#[inline(always)]
pub fn bool_expr(value: bool) -> Expr {
    Expr::Lit(Lit::Bool(Bool::from(value)))
}

#[inline(always)]
pub fn array_expr(elems: Vec<Option<ExprOrSpread>>) -> Expr {
    Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    })
}

#[inline(always)]
pub fn object_expr(props: Vec<PropOrSpread>) -> Expr {
    Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    })
}

#[inline(always)]
pub fn str_expr(value: Wtf8Atom) -> Expr {
    Expr::Lit(Lit::Str(Str::from(value)))
}

#[inline(always)]
pub fn prop_expr(expr: Expr) -> ExprOrSpread {
    ExprOrSpread::from(Box::new(expr))
}

#[inline(always)]
pub fn call_expr(callee: Expr, args: Vec<ExprOrSpread>) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(callee)),
        args,
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

#[inline(always)]
pub fn prop_ident(key: &str) -> PropName {
    PropName::Ident(IdentName::from(key))
}

#[inline(always)]
pub fn prop_key(key: &str) -> PropName {
    if is_valid_prop_ident(key) {
        prop_ident(key)
    } else {
        PropName::Str(Str::from(key))
    }
}

#[inline(always)]
pub fn prop(key: PropName, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key,
        value: Box::new(value),
    })))
}
