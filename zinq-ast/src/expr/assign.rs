use zinq_token::Ident;

use crate::{Node, Visitor, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
    pub name: Ident,
    pub value: Box<Expr>,
}

impl From<AssignExpr> for Expr {
    fn from(value: AssignExpr) -> Self {
        Self::Assign(value)
    }
}

impl Node for AssignExpr {
    fn name(&self) -> &str {
        "Expr::Assign"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", &self.name, &self.value)
    }
}
