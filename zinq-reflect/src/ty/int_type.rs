#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
}

impl std::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8 => write!(f, "Type::I8"),
            Self::I16 => write!(f, "Type::I16"),
            Self::I32 => write!(f, "Type::I32"),
            Self::I64 => write!(f, "Type::I64"),
        }
    }
}
