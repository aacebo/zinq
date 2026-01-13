use zinq_parse::{Parse, Peek, Spanned};
use zinq_token::DotDot;

use crate::{Node, pat::Pattern};

///
/// ## Spread Pattern
/// `..`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpreadPattern {
    pub dots: DotDot,
}

impl std::ops::Deref for SpreadPattern {
    type Target = DotDot;

    fn deref(&self) -> &Self::Target {
        &self.dots
    }
}

impl From<DotDot> for SpreadPattern {
    fn from(value: DotDot) -> Self {
        Self { dots: value }
    }
}

impl From<SpreadPattern> for Pattern {
    fn from(value: SpreadPattern) -> Self {
        Self::Spread(value)
    }
}

impl std::fmt::Display for SpreadPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for SpreadPattern {
    fn span(&self) -> zinq_parse::Span {
        self.dots.span()
    }
}

impl Peek for SpreadPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<DotDot>(cursor).unwrap_or(false))
    }
}

impl Parse for SpreadPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let dots = parser.parse::<DotDot>(cursor)?;
        Ok(Self { dots })
    }
}

impl Node for SpreadPattern {
    fn name(&self) -> &str {
        "Pattern::Spread"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_spread_pattern(self);
    }
}
