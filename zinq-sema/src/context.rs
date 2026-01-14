use crate::{Arena, ExprId, expr::Expr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemanticContext {
    exprs: Arena<ExprId, Expr>,
}
