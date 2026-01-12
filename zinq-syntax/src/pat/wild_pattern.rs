use zinq_parse::{Parse, Peek, Spanned};
use zinq_token::Underscore;

use crate::pat::Pattern;

///
/// ## Wild Pattern
/// `_ => ..` <br>
/// `fn test(_: &str)`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildPattern {
    pub underscore: Underscore,
}

impl std::ops::Deref for WildPattern {
    type Target = Underscore;

    fn deref(&self) -> &Self::Target {
        &self.underscore
    }
}

impl From<Underscore> for WildPattern {
    fn from(value: Underscore) -> Self {
        Self { underscore: value }
    }
}

impl From<WildPattern> for Pattern {
    fn from(value: WildPattern) -> Self {
        Self::Wild(value)
    }
}

impl std::fmt::Display for WildPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for WildPattern {
    fn span(&self) -> zinq_parse::Span {
        self.underscore.span()
    }
}

impl Peek for WildPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Underscore>(cursor).unwrap_or(false))
    }
}

impl Parse for WildPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let underscore = parser.parse::<Underscore>(cursor)?;
        Ok(Self { underscore })
    }
}
