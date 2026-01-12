use zinq_parse::{Parse, Peek, Spanned};

use crate::{UseGlob, UseGroup, UseName, UsePath};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UseSection {
    Path(UsePath),
    Name(UseName),
    Glob(UseGlob),
    Group(UseGroup),
}

impl UseSection {
    pub fn is_path(&self) -> bool {
        match self {
            Self::Path(_) => true,
            _ => false,
        }
    }

    pub fn is_name(&self) -> bool {
        match self {
            Self::Name(_) => true,
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

    pub fn as_path(&self) -> &UsePath {
        match self {
            Self::Path(v) => v,
            _ => panic!("expected UsePath"),
        }
    }

    pub fn as_name(&self) -> &UseName {
        match self {
            Self::Name(v) => v,
            _ => panic!("expected UseName"),
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

impl From<UsePath> for UseSection {
    fn from(value: UsePath) -> Self {
        Self::Path(value)
    }
}

impl std::fmt::Display for UseSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(v) => write!(f, "{}", v),
            Self::Name(v) => write!(f, "{}", v),
            Self::Glob(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for UseSection {
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

impl Parse for UseSection {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<UseGlob>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseGlob>(cursor)?.into());
        }

        if parser.peek::<UseGroup>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseGroup>(cursor)?.into());
        }

        if parser.peek::<UsePath>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UsePath>(cursor)?.into());
        }

        if parser.peek::<UseName>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<UseName>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }
}

impl Spanned for UseSection {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Path(v) => v.span(),
            Self::Name(v) => v.span(),
            Self::Glob(v) => v.span(),
            Self::Group(v) => v.span(),
        }
    }
}
