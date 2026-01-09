mod type_path;
mod use_path;

pub use type_path::*;
pub use use_path::*;

use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{ColonColon, Ident, Punctuated};

///
/// ## Path
/// `std::string::String`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    items: Punctuated<Ident, ColonColon>,
}

impl Path {
    /// check if is a len of 1 with no arguments
    pub fn is_ident(&self) -> bool {
        self.items.len() == 1
    }

    pub fn as_ident(&self) -> &Ident {
        self.items.last().expect("expected non empty path").value()
    }
}

impl std::ops::Deref for Path {
    type Target = Punctuated<Ident, ColonColon>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for Path {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for Path {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let items = parser.parse::<Punctuated<Ident, ColonColon>>(cursor)?;
        Ok(Self { items })
    }
}

impl Spanned for Path {
    fn span(&self) -> Span {
        self.items.span()
    }
}
