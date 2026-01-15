mod field_attr;
mod int_attr;
mod str_attr;

pub use field_attr::*;
pub use int_attr::*;
pub use str_attr::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attr {
    Int(IntAttr),
    Str(StrAttr),
    Field(FieldAttr),
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Field(v) => write!(f, "{}", v),
        }
    }
}
