mod index_fields;
mod name_fields;

pub use index_fields::*;
pub use name_fields::*;
use zinq_parse::{Parse, Peek, Span, Spanned};

use crate::Syntax;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Fields {
    None(Span),
    Indexed(IndexFields),
    Named(NameFields),
}

impl Fields {
    pub fn len(&self) -> usize {
        match self {
            Self::None(_) => 0,
            Self::Indexed(v) => v.len(),
            Self::Named(v) => v.len(),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::None(_) => true,
            _ => false,
        }
    }

    pub fn is_indexed(&self) -> bool {
        match self {
            Self::Indexed(_) => true,
            _ => false,
        }
    }

    pub fn is_named(&self) -> bool {
        match self {
            Self::Named(_) => true,
            _ => false,
        }
    }

    pub fn as_indexed(&self) -> &IndexFields {
        match self {
            Self::Indexed(v) => v,
            v => panic!("expected IndexFields, received {}", v.name()),
        }
    }

    pub fn as_named(&self) -> &NameFields {
        match self {
            Self::Named(v) => v,
            v => panic!("expected NameFields, received {}", v.name()),
        }
    }
}

impl Syntax for Fields {
    fn name(&self) -> &str {
        match self {
            Self::None(_) => "Fields::None",
            Self::Indexed(v) => v.name(),
            Self::Named(v) => v.name(),
        }
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None(_) => Ok(()),
            Self::Indexed(v) => write!(f, "{}", v),
            Self::Named(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Fields {
    fn peek(_: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
        Ok(true)
    }
}

impl Parse for Fields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<IndexFields>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<IndexFields>(cursor)?.into());
        }

        if parser.peek::<NameFields>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<NameFields>(cursor)?.into());
        }

        Ok(Self::None(cursor.span().clone()))
    }
}

impl Spanned for Fields {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::None(span) => span.clone(),
            Self::Indexed(v) => v.span(),
            Self::Named(v) => v.span(),
        }
    }
}
