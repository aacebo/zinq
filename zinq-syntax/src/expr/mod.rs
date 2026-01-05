mod binary;
mod postfix;
mod primary;
mod unary;

pub use binary::*;
pub use postfix::*;
pub use primary::*;
pub use unary::*;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::{Node, Syntax, Visitor};

///
/// ## Expression
/// Something that can be evaluated, usually to a value (even if that value is `()`).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Primary(PrimaryExpr),
    Binary(BinaryExpr),
    Postfix(PostfixExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn is_primary(&self) -> bool {
        match self {
            Self::Primary(_) => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Self::Binary(_) => true,
            _ => false,
        }
    }

    pub fn is_postfix(&self) -> bool {
        match self {
            Self::Postfix(_) => true,
            _ => false,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Self::Unary(_) => true,
            _ => false,
        }
    }
}

impl Node for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Primary(v) => v.name(),
            Self::Binary(v) => v.name(),
            Self::Postfix(v) => v.name(),
            Self::Unary(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl From<Expr> for Syntax {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::Postfix(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Expr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Expr {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut TokenParser) -> Result<Self> {
        if parser.peek_as::<PrimaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<PrimaryExpr>(cursor)?.into());
        }

        if parser.peek_as::<PostfixExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<PostfixExpr>(cursor)?.into());
        }

        if parser.peek_as::<UnaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<UnaryExpr>(cursor)?.into());
        }

        if parser.peek_as::<BinaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<BinaryExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Primary(v) => v.span(),
            Self::Binary(v) => v.span(),
            Self::Postfix(v) => v.span(),
            Self::Unary(v) => v.span(),
        }
    }
}
