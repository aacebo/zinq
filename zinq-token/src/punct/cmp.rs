use zinq_parse::{Parse, Parser, Peek};

use crate::{EqEq, Gt, GtEq, Lt, LtEq, NotEq, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmp {
    Eq(EqEq),
    NotEq(NotEq),
    Gt(Gt),
    GtEq(GtEq),
    Lt(Lt),
    LtEq(LtEq),
}

impl Cmp {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Eq(v) => v.name(),
            Self::NotEq(v) => v.name(),
            Self::Gt(v) => v.name(),
            Self::GtEq(v) => v.name(),
            Self::Lt(v) => v.name(),
            Self::LtEq(v) => v.name(),
        }
    }

    pub fn is_eq(&self) -> bool {
        match self {
            Self::Eq(_) => true,
            _ => false,
        }
    }

    pub fn is_not_eq(&self) -> bool {
        match self {
            Self::NotEq(_) => true,
            _ => false,
        }
    }

    pub fn is_gt(&self) -> bool {
        match self {
            Self::Gt(_) => true,
            _ => false,
        }
    }

    pub fn is_gt_eq(&self) -> bool {
        match self {
            Self::GtEq(_) => true,
            _ => false,
        }
    }

    pub fn is_lt(&self) -> bool {
        match self {
            Self::Lt(_) => true,
            _ => false,
        }
    }

    pub fn is_lt_eq(&self) -> bool {
        match self {
            Self::LtEq(_) => true,
            _ => false,
        }
    }
}

impl From<EqEq> for Cmp {
    fn from(value: EqEq) -> Self {
        Self::Eq(value)
    }
}

impl From<NotEq> for Cmp {
    fn from(value: NotEq) -> Self {
        Self::NotEq(value)
    }
}

impl From<Gt> for Cmp {
    fn from(value: Gt) -> Self {
        Self::Gt(value)
    }
}

impl From<GtEq> for Cmp {
    fn from(value: GtEq) -> Self {
        Self::GtEq(value)
    }
}

impl From<Lt> for Cmp {
    fn from(value: Lt) -> Self {
        Self::Lt(value)
    }
}

impl From<LtEq> for Cmp {
    fn from(value: LtEq) -> Self {
        Self::LtEq(value)
    }
}

impl std::fmt::Display for Cmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq(v) => write!(f, "{}", v),
            Self::NotEq(v) => write!(f, "{}", v),
            Self::Gt(v) => write!(f, "{}", v),
            Self::GtEq(v) => write!(f, "{}", v),
            Self::Lt(v) => write!(f, "{}", v),
            Self::LtEq(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Cmp {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<EqEq>(cursor).unwrap_or(false)
            || parser.peek_as::<NotEq>(cursor).unwrap_or(false)
            || parser.peek_as::<GtEq>(cursor).unwrap_or(false)
            || parser.peek_as::<Gt>(cursor).unwrap_or(false)
            || parser.peek_as::<LtEq>(cursor).unwrap_or(false)
            || parser.peek_as::<Lt>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for Cmp {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<EqEq>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<EqEq>(cursor)?.into());
        }

        if parser.peek_as::<NotEq>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<NotEq>(cursor)?.into());
        }

        if parser.peek_as::<GtEq>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<GtEq>(cursor)?.into());
        }

        if parser.peek_as::<Gt>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<Gt>(cursor)?.into());
        }

        if parser.peek_as::<LtEq>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<LtEq>(cursor)?.into());
        }

        if parser.peek_as::<Lt>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<Lt>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Eq(v) => v.span(),
            Self::NotEq(v) => v.span(),
            Self::Gt(v) => v.span(),
            Self::GtEq(v) => v.span(),
            Self::Lt(v) => v.span(),
            Self::LtEq(v) => v.span(),
        }
    }
}
