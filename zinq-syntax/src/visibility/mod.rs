mod mod_visibility;
mod private_visibility;
mod public_visibility;
mod super_visibility;

pub use mod_visibility::*;
pub use private_visibility::*;
pub use public_visibility::*;
pub use super_visibility::*;
use zinq_error::Result;
use zinq_parse::{Parse, Peek};

use crate::{Node, Syntax, Visitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Pub(PublicVisibility),
    Super(SuperVisibility),
    Mod(ModVisibility),
    Priv(PrivateVisibility),
}

impl Visibility {
    pub fn is_pub(&self) -> bool {
        match self {
            Self::Pub(_) => true,
            _ => false,
        }
    }

    pub fn is_super(&self) -> bool {
        match self {
            Self::Super(_) => true,
            _ => false,
        }
    }

    pub fn is_mod(&self) -> bool {
        match self {
            Self::Mod(_) => true,
            _ => false,
        }
    }

    pub fn is_priv(&self) -> bool {
        match self {
            Self::Priv(_) => true,
            _ => false,
        }
    }
}

impl From<Visibility> for Syntax {
    fn from(value: Visibility) -> Self {
        Self::Visibility(value)
    }
}

impl Node for Visibility {
    fn name(&self) -> &str {
        match self {
            Self::Pub(v) => v.name(),
            Self::Mod(v) => v.name(),
            Self::Super(v) => v.name(),
            Self::Priv(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pub(v) => write!(f, "{}", v),
            Self::Mod(v) => write!(f, "{}", v),
            Self::Super(v) => write!(f, "{}", v),
            Self::Priv(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Visibility {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Visibility {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<ModVisibility>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<ModVisibility>(cursor)?.into());
        }

        if parser.peek::<SuperVisibility>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<SuperVisibility>(cursor)?.into());
        }

        if parser.peek::<PublicVisibility>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<PublicVisibility>(cursor)?.into());
        }

        Ok(parser.parse::<PrivateVisibility>(cursor)?.into())
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Pub(v) => v.span(),
            Self::Mod(v) => v.span(),
            Self::Super(v) => v.span(),
            Self::Priv(v) => v.span(),
        }
    }
}
