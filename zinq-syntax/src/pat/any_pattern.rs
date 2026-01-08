use zinq_parse::{Parse, Peek, Spanned};
use zinq_token::Underscore;

use crate::pat::Pattern;

///
/// ## Any Pattern
/// `_ => ..` <br>
/// `fn test(_: &str)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyPattern {
    pub underscore: Underscore,
}

impl std::ops::Deref for AnyPattern {
    type Target = Underscore;

    fn deref(&self) -> &Self::Target {
        &self.underscore
    }
}

impl From<Underscore> for AnyPattern {
    fn from(value: Underscore) -> Self {
        Self { underscore: value }
    }
}

impl From<AnyPattern> for Pattern {
    fn from(value: AnyPattern) -> Self {
        Self::Any(value)
    }
}

impl std::fmt::Display for AnyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for AnyPattern {
    fn span(&self) -> zinq_parse::Span {
        self.underscore.span()
    }
}

impl Peek for AnyPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Underscore>(cursor).unwrap_or(false))
    }
}

impl Parse for AnyPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let underscore = parser.parse::<Underscore>(cursor)?;
        Ok(Self { underscore })
    }
}
