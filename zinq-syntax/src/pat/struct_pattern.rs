use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, DotDot, Ident, LBrace, Punctuated, RBrace};

use crate::{TypePath, pat::Pattern};

///
/// ## Struct Pattern
/// `MyStruct { a, b, .. }`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructPattern {
    pub path: TypePath,
    pub left_brace: LBrace,
    pub fields: Punctuated<Ident, Comma>,
    pub spread: Option<DotDot>,
    pub right_brace: RBrace,
}

impl From<StructPattern> for Pattern {
    fn from(value: StructPattern) -> Self {
        Self::Struct(value)
    }
}

impl std::fmt::Display for StructPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for StructPattern {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.path.span(), self.right_brace.span())
    }
}

impl Peek for StructPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<TypePath>(cursor).unwrap_or(false))
    }
}

impl Parse for StructPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<TypePath>(cursor)?;
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let fields = parser.parse::<Punctuated<Ident, Comma>>(cursor)?;
        let spread = parser.parse::<Option<DotDot>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            path,
            left_brace,
            fields,
            spread,
            right_brace,
        })
    }
}
