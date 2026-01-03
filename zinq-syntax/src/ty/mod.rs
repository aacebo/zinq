mod mut_type;
mod path_type;
mod ref_type;
mod struct_type;

pub use mut_type::*;
pub use path_type::*;
pub use ref_type::*;
pub use struct_type::*;
use zinq_parse::{Parse, Parser, Peek};

use crate::{Node, Syntax, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Path(PathType),
    Mut(MutType),
    Ref(RefType),
    Struct(StructType),
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
            Self::Struct(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
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
            Self::Struct(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Type {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
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
        if parser.peek_as::<PathType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<PathType>(cursor)?.into());
        }

        if parser.peek_as::<RefType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<RefType>(cursor)?.into());
        }

        if parser.peek_as::<MutType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<MutType>(cursor)?.into());
        }

        if parser.peek_as::<StructType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<StructType>(cursor)?.into());
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
            Self::Struct(v) => v.span(),
        }
    }
}
