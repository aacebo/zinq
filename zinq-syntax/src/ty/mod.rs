mod mut_type;
mod path_type;
mod ref_type;
mod slice_type;
mod tuple_type;
mod visitor;

pub use mut_type::*;
pub use path_type::*;
pub use ref_type::*;
pub use slice_type::*;
pub use tuple_type::*;
pub use visitor::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek, Spanned};

use crate::Syntax;

///
/// ## Type
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Path(PathType),
    Mut(MutType),
    Ref(RefType),
    Slice(SliceType),
    Tuple(TupleType),
}

impl Syntax for Type {
    fn name(&self) -> &str {
        match self {
            Self::Path(v) => v.name(),
            Self::Mut(v) => v.name(),
            Self::Ref(v) => v.name(),
            Self::Slice(v) => v.name(),
            Self::Tuple(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_type(self);

        match self {
            Self::Mut(v) => v.accept(visitor),
            Self::Path(v) => v.accept(visitor),
            Self::Ref(v) => v.accept(visitor),
            Self::Slice(v) => v.accept(visitor),
            Self::Tuple(v) => v.accept(visitor),
        }
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

impl Peek for Type {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Type {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<RefType>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<RefType>(cursor)?.into());
        }

        if parser.peek::<MutType>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<MutType>(cursor)?.into());
        }

        if parser.peek::<PathType>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<PathType>(cursor)?.into());
        }

        if parser.peek::<SliceType>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<SliceType>(cursor)?.into());
        }

        if parser.peek::<TupleType>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<TupleType>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unexpected token '{}'", *cursor.peek()? as char),
        ))
    }
}

impl Spanned for Type {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Path(v) => v.span(),
            Self::Mut(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Slice(v) => v.span(),
            Self::Tuple(v) => v.span(),
        }
    }
}
