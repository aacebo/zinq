use zinq_token::{LParen, RParen};

use crate::{Node, Visitor, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpr {
    pub target: Box<Expr>,
    pub left_paren: LParen,
    pub args: Vec<Expr>,
    pub right_paren: RParen,
}

impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}

impl Node for CallExpr {
    fn name(&self) -> &str {
        "Expr::Call"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", &self.target, &self.left_paren)?;

        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i < self.args.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "{}", &self.right_paren)
    }
}
