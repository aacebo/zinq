use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Colon, Plus, Punctuated};

use crate::TypePath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounds {
    pub colon: Colon,
    pub items: Punctuated<TypePath, Plus>,
}

impl std::fmt::Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for Bounds {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.colon.span(), self.items.span())
    }
}

impl Peek for Bounds {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Colon>(cursor).unwrap_or(false))
    }
}

impl Parse for Bounds {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let colon = parser.parse::<Colon>(cursor)?;
        let items = parser.parse::<Punctuated<TypePath, Plus>>(cursor)?;

        Ok(Self { colon, items })
    }
}
