pub mod expr;

use zinq_error::Result;

use crate::expr::Expr;

pub trait Node {
    fn name(&self) -> &str;
    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized;
}

pub trait Visitor<N: Node> {
    fn visit(&mut self, node: &N) -> Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Any {
    Expr(Expr),
}

impl std::fmt::Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(v) => write!(f, "{}", v),
        }
    }
}

impl Node for Any {
    fn name(&self) -> &str {
        match self {
            Self::Expr(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}
