mod literal_pattern;
mod parser;
mod path_pattern;
mod ref_pattern;
mod spread_pattern;
mod struct_pattern;
mod tuple_pattern;
mod wild_pattern;

pub use literal_pattern::*;
pub use parser::*;
pub use path_pattern::*;
pub use ref_pattern::*;
pub use spread_pattern::*;
pub use struct_pattern::*;
pub use tuple_pattern::*;
pub use wild_pattern::*;

use zinq_parse::{Parse, Peek, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Wild(WildPattern),
    Path(PathPattern),
    Literal(LiteralPattern),
    Ref(RefPattern),
    Spread(SpreadPattern),
    Struct(StructPattern),
    Tuple(TuplePattern),
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wild(v) => write!(f, "{}", v),
            Self::Path(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Spread(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
        }
    }
}

impl Spanned for Pattern {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Wild(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Spread(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Tuple(v) => v.span(),
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
        parser.parse_pattern(cursor)
    }
}
