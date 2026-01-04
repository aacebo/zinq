mod r#mut;
mod r#ref;
mod slice;
mod tuple;

pub use r#mut::*;
pub use r#ref::*;
pub use slice::*;
pub use tuple::*;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::{Node, Path, Syntax, Visitor};

///
/// ## Type
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Path(Path),
    Mut(MutType),
    Ref(RefType),
    Slice(SliceType),
    Tuple(TupleType),
}

impl From<Type> for Syntax {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

impl Node for Type {
    fn name(&self) -> &str {
        match self {
            Self::Path(v) => v.name(),
            Self::Mut(v) => v.name(),
            Self::Ref(v) => v.name(),
            Self::Slice(v) => v.name(),
            Self::Tuple(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(v) => write!(f, "{}", v),
            Self::Mut(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Type {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Type {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<RefType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<RefType>(cursor)?.into());
        }

        if parser.peek_as::<MutType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<MutType>(cursor)?.into());
        }

        if parser.peek_as::<Path>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<Path>(cursor)?.into());
        }

        if parser.peek_as::<SliceType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<SliceType>(cursor)?.into());
        }

        if parser.peek_as::<TupleType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<TupleType>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Path(v) => v.span(),
            Self::Mut(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Slice(v) => v.span(),
            Self::Tuple(v) => v.span(),
        }
    }
}
