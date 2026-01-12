use zinq_parse::{Parse, Peek, Spanned};

use crate::{Minus, Plus, Slash, Star};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arithmetic {
    Add(Plus),
    Sub(Minus),
    Div(Slash),
    Mult(Star),
}

impl Arithmetic {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Add(v) => v.name(),
            Self::Sub(v) => v.name(),
            Self::Div(v) => v.name(),
            Self::Mult(v) => v.name(),
        }
    }

    pub fn is_add(&self) -> bool {
        match self {
            Self::Add(_) => true,
            _ => false,
        }
    }

    pub fn is_sub(&self) -> bool {
        match self {
            Self::Sub(_) => true,
            _ => false,
        }
    }

    pub fn is_div(&self) -> bool {
        match self {
            Self::Div(_) => true,
            _ => false,
        }
    }

    pub fn is_mult(&self) -> bool {
        match self {
            Self::Mult(_) => true,
            _ => false,
        }
    }
}

impl From<Plus> for Arithmetic {
    fn from(value: Plus) -> Self {
        Self::Add(value)
    }
}

impl From<Minus> for Arithmetic {
    fn from(value: Minus) -> Self {
        Self::Sub(value)
    }
}

impl From<Slash> for Arithmetic {
    fn from(value: Slash) -> Self {
        Self::Div(value)
    }
}

impl From<Star> for Arithmetic {
    fn from(value: Star) -> Self {
        Self::Mult(value)
    }
}

impl std::fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(v) => write!(f, "{}", v),
            Self::Sub(v) => write!(f, "{}", v),
            Self::Div(v) => write!(f, "{}", v),
            Self::Mult(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Arithmetic {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Plus>(cursor).unwrap_or(false)
            || parser.peek::<Minus>(cursor).unwrap_or(false)
            || parser.peek::<Slash>(cursor).unwrap_or(false)
            || parser.peek::<Star>(cursor).unwrap_or(false))
    }
}

impl Parse for Arithmetic {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<Plus>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Plus>(cursor)?.into());
        }

        if parser.peek::<Minus>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Minus>(cursor)?.into());
        }

        if parser.peek::<Slash>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Plus>(cursor)?.into());
        }

        if parser.peek::<Star>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Minus>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unexpected token '{}'", *cursor.peek()? as char),
        ))
    }
}

impl Spanned for Arithmetic {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Add(v) => v.span(),
            Self::Sub(v) => v.span(),
            Self::Div(v) => v.span(),
            Self::Mult(v) => v.span(),
        }
    }
}
