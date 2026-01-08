use zinq_parse::Spanned;
use zinq_token::Literal;

use crate::pat::Pattern;

///
/// ## Literal Pattern
/// `0 => ..` <br>
/// `"test" => ..`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralPattern {
    pub literal: Literal,
}

impl std::ops::Deref for LiteralPattern {
    type Target = Literal;

    fn deref(&self) -> &Self::Target {
        &self.literal
    }
}

impl From<Literal> for LiteralPattern {
    fn from(value: Literal) -> Self {
        Self { literal: value }
    }
}

impl From<LiteralPattern> for Pattern {
    fn from(value: LiteralPattern) -> Self {
        Self::Literal(value)
    }
}

impl std::fmt::Display for LiteralPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for LiteralPattern {
    fn span(&self) -> zinq_parse::Span {
        self.literal.span()
    }
}
