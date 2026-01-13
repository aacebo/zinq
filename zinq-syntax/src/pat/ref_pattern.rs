use zinq_parse::{Span, Spanned};
use zinq_token::{And, Mut};

use crate::{Node, pat::Pattern};

///
/// ## Reference Pattern
/// - `&a`
/// - `&mut a`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Node for RefPattern {
    fn name(&self) -> &str {
        "Pattern::Ref"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_ref_pattern(self);
        self.inner.accept(visitor);
    }
}
