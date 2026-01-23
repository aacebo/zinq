use crate::id::ExprId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirUnaryExpr {
    pub op: UnaryOp,
    pub right: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Negative,
    Not,
}
