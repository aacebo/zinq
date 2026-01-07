use zinq_parse::{Parse, Peek, Span};
use zinq_token::Ident;

use crate::path::UseSegment;

///
/// ## Use Ident
/// `use std::string::String;`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseIdent {
    pub ident: Ident,
}

impl From<UseIdent> for UseSegment {
    fn from(value: UseIdent) -> Self {
        Self::Ident(value)
    }
}

impl std::fmt::Display for UseIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident.span())
    }
}

impl Peek for UseIdent {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for UseIdent {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;

        Ok(Self { ident })
    }

    fn span(&self) -> &Span {
        self.ident.span()
    }
}
