use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LParen, Punctuated, RParen};

use crate::pat::Pattern;

///
/// ## Tuple Pattern
/// `(a, b)`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePattern {
    pub left_paren: LParen,
    pub items: Punctuated<Pattern, Comma>,
    pub right_paren: RParen,
}

impl From<TuplePattern> for Pattern {
    fn from(value: TuplePattern) -> Self {
        Self::Tuple(value)
    }
}

impl std::fmt::Display for TuplePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for TuplePattern {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.left_paren.span(), self.right_paren.span())
    }
}

impl Peek for TuplePattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LParen>(cursor).unwrap_or(false))
    }
}

impl Parse for TuplePattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let items = parser.parse::<Punctuated<Pattern, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            left_paren,
            items,
            right_paren,
        })
    }
}
