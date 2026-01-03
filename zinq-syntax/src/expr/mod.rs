mod assign;
mod binary;
mod group;
mod invoke;
mod literal;
mod resolve;
mod resolve_field;

pub use assign::*;
pub use binary::*;
pub use group::*;
pub use invoke::*;
pub use literal::*;
pub use resolve::*;
pub use resolve_field::*;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::{Node, Syntax, Visitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Group(GroupExpr),
    Invoke(InvokeExpr),
    Literal(LiteralExpr),
    ResolveField(ResolveFieldExpr),
    Resolve(ResolveExpr),
}

impl Node for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Assign(v) => v.name(),
            Self::Binary(v) => v.name(),
            Self::Group(v) => v.name(),
            Self::Invoke(v) => v.name(),
            Self::Literal(v) => v.name(),
            Self::ResolveField(v) => v.name(),
            Self::Resolve(v) => v.name(),
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
        Syntax::Expr(value)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Invoke(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::ResolveField(v) => write!(f, "{}", v),
            Self::Resolve(v) => write!(f, "{}", v),
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
        if parser.peek_as::<LiteralExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<LiteralExpr>(cursor)?.into());
        }

        if parser.peek_as::<ResolveExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<ResolveExpr>(cursor)?.into());
        }

        if parser.peek_as::<ResolveFieldExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<ResolveFieldExpr>(cursor)?.into());
        }

        if parser.peek_as::<AssignExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<AssignExpr>(cursor)?.into());
        }

        if parser.peek_as::<BinaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<BinaryExpr>(cursor)?.into());
        }

        if parser.peek_as::<InvokeExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<InvokeExpr>(cursor)?.into());
        }

        if parser.peek_as::<GroupExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<GroupExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Assign(v) => v.span(),
            Self::Binary(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Invoke(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::ResolveField(v) => v.span(),
            Self::Resolve(v) => v.span(),
        }
    }
}
