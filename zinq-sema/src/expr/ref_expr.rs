use crate::ExprId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub inner: ExprId,
}
