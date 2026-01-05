mod get;
mod get_field;
mod group;
mod literal;

pub use get::*;
pub use get_field::*;
pub use group::*;
pub use literal::*;

use zinq_parse::ZinqParser;
use zinq_parse::{Parse, Parser, Peek};

use crate::{Node, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpr {
    Literal(LiteralExpr),
    Get(GetExpr),
    GetField(GetFieldExpr),
    Group(GroupExpr),
}

impl PrimaryExpr {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_get(&self) -> bool {
        match self {
            Self::Get(_) => true,
            _ => false,
        }
    }

    pub fn is_get_field(&self) -> bool {
        match self {
            Self::GetField(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }
}

impl From<PrimaryExpr> for Expr {
    fn from(value: PrimaryExpr) -> Self {
        Self::Primary(value)
    }
}

impl std::fmt::Display for PrimaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{}", v),
            Self::Get(v) => write!(f, "{}", v),
            Self::GetField(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl Node for PrimaryExpr {
    fn name(&self) -> &str {
        match self {
            Self::Literal(v) => v.name(),
            Self::Get(v) => v.name(),
            Self::GetField(v) => v.name(),
            Self::Group(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek for PrimaryExpr {
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

impl Parse for PrimaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<LiteralExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<LiteralExpr>(cursor)?.into());
        }

        if parser.peek::<GetFieldExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<GetFieldExpr>(cursor)?.into());
        }

        if parser.peek::<GetExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<GetExpr>(cursor)?.into());
        }

        if parser.peek::<GroupExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<GroupExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Literal(v) => v.span(),
            Self::Get(v) => v.span(),
            Self::GetField(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}
