mod mut_type;
mod ref_type;
mod struct_type;

pub use mut_type::*;
pub use ref_type::*;
pub use struct_type::*;
use zinq_parse::{Parse, Parser, Peek};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Mut(MutType),
    Ref(RefType),
    Struct(StructType),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mut(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Type {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<StructType>(cursor).unwrap_or(false)
            || parser.peek_as::<MutType>(cursor).unwrap_or(false)
            || parser.peek_as::<RefType>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for Type {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<MutType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<MutType>(cursor)?.into());
        }

        if parser.peek_as::<RefType>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<RefType>(cursor)?.into());
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
            Self::Mut(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Struct(v) => v.span(),
        }
    }
}
