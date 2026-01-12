use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::Ident;

use crate::use_path::UseSection;

///
/// ## Use Name
/// - `use std::string::String`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseName {
    pub ident: Ident,
}

impl From<UseName> for UseSection {
    fn from(value: UseName) -> Self {
        Self::Name(value)
    }
}

impl std::fmt::Display for UseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl std::ops::Deref for UseName {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.ident
    }
}

impl Peek for UseName {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for UseName {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;

        Ok(Self { ident })
    }
}

impl Spanned for UseName {
    fn span(&self) -> Span {
        self.ident.span()
    }
}
