use zinq_parse::{Span, Spanned};
use zinq_token::Or;

use crate::{Node, pat::Pattern};

///
/// ## Or Pattern
/// `A | B`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrPattern {
    pub left: Box<Pattern>,
    pub or: Or,
    pub right: Box<Pattern>,
}

impl From<OrPattern> for Pattern {
    fn from(value: OrPattern) -> Self {
        Self::Or(value)
    }
}

impl std::fmt::Display for OrPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for OrPattern {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.left.span(), self.right.span())
    }
}

impl Node for OrPattern {
    fn name(&self) -> &str {
        "Pattern::Or"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_or_pattern(self);
        self.left.accept(visitor);
        self.right.accept(visitor);
    }
}
