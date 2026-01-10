use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{LParen, RParen};

use crate::pat::Pattern;

///
/// ## Group Pattern
/// `(...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupPattern {
    pub left_paren: LParen,
    pub inner: Box<Pattern>,
    pub right_paren: RParen,
}

impl From<GroupPattern> for Pattern {
    fn from(value: GroupPattern) -> Self {
        Self::Group(value)
    }
}

impl std::fmt::Display for GroupPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for GroupPattern {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.left_paren.span(), self.right_paren.span())
    }
}

impl Peek for GroupPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LParen>(cursor).unwrap_or(false))
    }
}

impl Parse for GroupPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let inner = parser.parse::<Box<Pattern>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            left_paren,
            inner,
            right_paren,
        })
    }
}
