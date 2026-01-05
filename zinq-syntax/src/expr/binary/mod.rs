mod assign;
mod cmp;
mod logical;

pub use assign::*;
pub use cmp::*;
pub use logical::*;
use zinq_parse::ZinqParser;
use zinq_parse::{Parse, Parser, Peek};

use crate::{Node, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpr {
    Assign(AssignExpr),
    Cmp(CmpExpr),
    Logical(LogicalExpr),
}

impl BinaryExpr {
    pub fn is_assign(&self) -> bool {
        match self {
            Self::Assign(_) => true,
            _ => false,
        }
    }

    pub fn is_cmp(&self) -> bool {
        match self {
            Self::Cmp(_) => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Self::Logical(_) => true,
            _ => false,
        }
    }
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}

impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Cmp(v) => write!(f, "{}", v),
            Self::Logical(v) => write!(f, "{}", v),
        }
    }
}

impl Node for BinaryExpr {
    fn name(&self) -> &str {
        match self {
            Self::Assign(v) => v.name(),
            Self::Cmp(v) => v.name(),
            Self::Logical(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek for BinaryExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for BinaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<CmpExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<CmpExpr>(cursor)?.into());
        }

        if parser.peek::<LogicalExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<LogicalExpr>(cursor)?.into());
        }

        if parser.peek::<AssignExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<AssignExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Assign(v) => v.span(),
            Self::Cmp(v) => v.span(),
            Self::Logical(v) => v.span(),
        }
    }
}
