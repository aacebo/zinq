use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{And, Mut};

use crate::pat::Pattern;

///
/// ## Reference Pattern
/// - `&a`
/// - `&mut a`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefPattern {
    pub and: And,
    pub mutable: Option<Mut>,
    pub inner: Box<Pattern>,
}

impl From<RefPattern> for Pattern {
    fn from(value: RefPattern) -> Self {
        Self::Ref(value)
    }
}

impl std::fmt::Display for RefPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for RefPattern {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.and.span(), self.inner.span())
    }
}

impl Peek for RefPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<And>(cursor).unwrap_or(false))
    }
}

impl Parse for RefPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse::<And>(cursor)?;
        let mutable = parser.parse::<Option<Mut>>(cursor)?;
        let inner = parser.parse::<Box<Pattern>>(cursor)?;

        Ok(Self {
            and,
            mutable,
            inner,
        })
    }
}
