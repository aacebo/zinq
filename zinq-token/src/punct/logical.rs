use zinq_parse::{Parse, Peek, Spanned};

use crate::{AndAnd, OrOr};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Logical {
    And(AndAnd),
    Or(OrOr),
}

impl Logical {
    pub fn name(&self) -> &'static str {
        match self {
            Self::And(v) => v.name(),
            Self::Or(v) => v.name(),
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            Self::And(_) => true,
            _ => false,
        }
    }

    pub fn is_or(&self) -> bool {
        match self {
            Self::Or(_) => true,
            _ => false,
        }
    }
}

impl From<AndAnd> for Logical {
    fn from(value: AndAnd) -> Self {
        Self::And(value)
    }
}

impl From<OrOr> for Logical {
    fn from(value: OrOr) -> Self {
        Self::Or(value)
    }
}

impl std::fmt::Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And(v) => write!(f, "{}", v),
            Self::Or(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Logical {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<AndAnd>(cursor).unwrap_or(false)
            || parser.peek::<OrOr>(cursor).unwrap_or(false))
    }
}

impl Parse for Logical {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<AndAnd>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<AndAnd>(cursor)?.into());
        }

        if parser.peek::<OrOr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<OrOr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unexpected token '{}'", *cursor.peek()? as char),
        ))
    }
}

impl Spanned for Logical {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::And(v) => v.span(),
            Self::Or(v) => v.span(),
        }
    }
}
