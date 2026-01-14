use crate::ExprId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemaRefExpr {
    pub inner: ExprId,
}
