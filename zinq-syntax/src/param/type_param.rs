use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::Ident;

use crate::Bounds;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub ident: Ident,
    pub bounds: Option<Bounds>,
}

impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for TypeParam {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for TypeParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;
        let bounds = parser.parse::<Option<Bounds>>(cursor)?;

        Ok(Self { ident, bounds })
    }
}

impl Spanned for TypeParam {
    fn span(&self) -> Span {
        if let Some(bounds) = &self.bounds {
            return Span::join(self.ident.span(), bounds.span());
        }

        self.ident.span()
    }
}
