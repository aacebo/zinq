use zinq_parse::{Parse, Peek, Spanned};
use zinq_token::Ident;

use crate::pat::Pattern;

///
/// ## Ident Pattern
/// `a`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentPattern {
    pub ident: Ident,
}

impl From<IdentPattern> for Pattern {
    fn from(value: IdentPattern) -> Self {
        Self::Ident(value)
    }
}

impl std::fmt::Display for IdentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for IdentPattern {
    fn span(&self) -> zinq_parse::Span {
        self.ident.span()
    }
}

impl Peek for IdentPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for IdentPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;
        Ok(Self { ident })
    }
}
