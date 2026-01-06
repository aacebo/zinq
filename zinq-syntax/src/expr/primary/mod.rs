mod group;
mod ident;
mod literal;

pub use group::*;
pub use ident::*;
pub use literal::*;

use zinq_parse::{Parse, Peek};

use crate::{Node, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpr {
    Literal(LiteralExpr),
    Ident(IdentExpr),
    Group(GroupExpr),
}

impl PrimaryExpr {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Self::Ident(_) => true,
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
            Self::Ident(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl Node for PrimaryExpr {
    fn name(&self) -> &str {
        match self {
            Self::Literal(v) => v.name(),
            Self::Ident(v) => v.name(),
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

        if parser.peek::<IdentExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<IdentExpr>(cursor)?.into());
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
            Self::Ident(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}
