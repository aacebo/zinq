mod indexed;
mod named;

pub use indexed::*;
pub use named::*;
use zinq_parse::{Parse, Parser, Peek};

use crate::{Node, TokenParser, Visitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fields {
    Indexed(IndexedFields),
    Named(NamedFields),
}

impl Fields {
    pub fn len(&self) -> usize {
        match self {
            Self::Indexed(v) => v.len(),
            Self::Named(v) => v.len(),
        }
    }
}

impl Node for Fields {
    fn name(&self) -> &str {
        match self {
            Self::Indexed(v) => v.name(),
            Self::Named(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Indexed(v) => write!(f, "{}", v),
            Self::Named(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Fields {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Fields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<IndexedFields>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<IndexedFields>(cursor)?.into());
        }

        if parser.peek_as::<NamedFields>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<NamedFields>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Indexed(v) => v.span(),
            Self::Named(v) => v.span(),
        }
    }
}
