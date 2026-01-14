use crate::ExprId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub left: ExprId,
    pub op: BinaryOp,
    pub right: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Arithmetic,
    Compare,
    Equality,
    Logical,
}
