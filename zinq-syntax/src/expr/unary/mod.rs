mod addr;
mod not;

pub use addr::*;
pub use not::*;
use zinq_parse::{Parse, Peek};

use crate::{Node, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpr {
    Addr(AddrExpr),
    Not(NotExpr),
}

impl UnaryExpr {
    pub fn is_addr(&self) -> bool {
        match self {
            Self::Addr(_) => true,
            _ => false,
        }
    }

    pub fn is_not(&self) -> bool {
        match self {
            Self::Not(_) => true,
            _ => false,
        }
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addr(v) => write!(f, "{}", v),
            Self::Not(v) => write!(f, "{}", v),
        }
    }
}

impl Node for UnaryExpr {
    fn name(&self) -> &str {
        match self {
            Self::Addr(v) => v.name(),
            Self::Not(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek for UnaryExpr {
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

impl Parse for UnaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<AddrExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<AddrExpr>(cursor)?.into());
        }

        if parser.peek::<NotExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<NotExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Addr(v) => v.span(),
            Self::Not(v) => v.span(),
        }
    }
}
