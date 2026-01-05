mod addr;
mod not;

pub use addr::*;
pub use not::*;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

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

impl Peek<TokenParser> for UnaryExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for UnaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<AddrExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<AddrExpr>(cursor)?.into());
        }

        if parser.peek_as::<NotExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<NotExpr>(cursor)?.into());
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
