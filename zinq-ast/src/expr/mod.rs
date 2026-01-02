mod assign;
mod binary;
mod call;

pub use assign::*;
pub use binary::*;
pub use call::*;

use crate::{Node, Visitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

impl Node for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Assign(v) => v.name(),
            Self::Binary(v) => v.name(),
            Self::Call(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::Call(v) => write!(f, "{}", v),
        }
    }
}
