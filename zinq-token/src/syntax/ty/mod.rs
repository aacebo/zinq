mod struct_type;

pub use struct_type::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Struct(StructType),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(v) => write!(f, "{}", v),
        }
    }
}
