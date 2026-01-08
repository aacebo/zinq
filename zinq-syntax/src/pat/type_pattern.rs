use zinq_parse::Spanned;

use crate::{TypePath, pat::Pattern};

///
/// ## Type Pattern
/// `std::string::String => ..` <br>
/// `u8 => ..`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePattern {
    pub path: TypePath,
}

impl std::ops::Deref for TypePattern {
    type Target = TypePath;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl From<TypePath> for TypePattern {
    fn from(value: TypePath) -> Self {
        Self { path: value }
    }
}

impl From<TypePattern> for Pattern {
    fn from(value: TypePattern) -> Self {
        Self::Type(value)
    }
}

impl std::fmt::Display for TypePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for TypePattern {
    fn span(&self) -> zinq_parse::Span {
        self.path.span()
    }
}
