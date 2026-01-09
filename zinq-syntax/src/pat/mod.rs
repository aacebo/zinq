mod any_pattern;
mod literal_pattern;
mod struct_pattern;
mod tuple_pattern;
mod type_pattern;

pub use any_pattern::*;
pub use literal_pattern::*;
pub use struct_pattern::*;
pub use tuple_pattern::*;
pub use type_pattern::*;

use zinq_parse::{Parse, Peek, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Any(AnyPattern),
    Literal(LiteralPattern),
    Struct(StructPattern),
    Tuple(TuplePattern),
    Type(TypePattern),
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
            Self::Type(v) => write!(f, "{}", v),
        }
    }
}

impl Spanned for Pattern {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Any(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Type(v) => v.span(),
        }
    }
}

impl Peek for Pattern {
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

impl Parse for Pattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<AnyPattern>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<AnyPattern>(cursor)?.into());
        }

        if parser.peek::<LiteralPattern>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<LiteralPattern>(cursor)?.into());
        }

        if parser.peek::<TuplePattern>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<TuplePattern>(cursor)?.into());
        }

        if parser.peek::<StructPattern>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<StructPattern>(cursor)?.into());
        }

        if parser.peek::<TypePattern>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<TypePattern>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }
}
