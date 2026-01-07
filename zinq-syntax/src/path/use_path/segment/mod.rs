mod glob_segment;
mod group_segment;
mod ident_segment;

pub use glob_segment::*;
pub use group_segment::*;
pub use ident_segment::*;
use zinq_parse::{Parse, Peek};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseSegment {
    Ident(UseIdent),
    Glob(UseGlob),
    Group(UseGroup),
}

impl UseSegment {
    pub fn is_ident(&self) -> bool {
        match self {
            Self::Ident(_) => true,
            _ => false,
        }
    }

    pub fn is_glob(&self) -> bool {
        match self {
            Self::Glob(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn as_ident(&self) -> &UseIdent {
        match self {
            Self::Ident(v) => v,
            _ => panic!("expected UseIdent"),
        }
    }

    pub fn as_glob(&self) -> &UseGlob {
        match self {
            Self::Glob(v) => v,
            _ => panic!("expected UseGlob"),
        }
    }

    pub fn as_group(&self) -> &UseGroup {
        match self {
            Self::Group(v) => v,
            _ => panic!("expected UseGroup"),
        }
    }
}

impl std::fmt::Display for UseSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Glob(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for UseSegment {
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

impl Parse for UseSegment {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<UseIdent>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseIdent>(cursor)?.into());
        }

        if parser.peek::<UseGlob>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseGlob>(cursor)?.into());
        }

        if parser.peek::<UseGroup>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseGroup>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Glob(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}
