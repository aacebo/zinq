use crate::ExprId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemaUnaryExpr {
    pub op: UnaryOp,
    pub right: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Negative,
    Not,
}
