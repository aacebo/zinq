mod binary;
mod parser;
mod postfix;
mod primary;
mod unary;

pub use binary::*;
pub use parser::*;
pub use postfix::*;
pub use primary::*;
pub use unary::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek};

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

impl Peek for Expr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Expr {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut zinq_parse::ZinqParser) -> Result<Self> {
        if parser.peek::<PrimaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<PrimaryExpr>(cursor)?.into());
        }

        if parser.peek::<PostfixExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<PostfixExpr>(cursor)?.into());
        }

        if parser.peek::<UnaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UnaryExpr>(cursor)?.into());
        }

        if parser.peek::<BinaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<BinaryExpr>(cursor)?.into());
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
