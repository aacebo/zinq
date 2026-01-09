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
    pub path: Punctuated<Ident, ColonColon>,
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
        let path = parser.parse::<Punctuated<Ident, ColonColon>>(cursor)?;
        Ok(Self { path })
    }
}

impl Spanned for Path {
    fn span(&self) -> Span {
        self.path.span()
    }
}
