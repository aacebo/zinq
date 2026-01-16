#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIntType {
    U8,
    U16,
    U32,
    U64,
}

impl std::fmt::Display for UIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8 => write!(f, "Type::U8"),
            Self::U16 => write!(f, "Type::U16"),
            Self::U32 => write!(f, "Type::U32"),
            Self::U64 => write!(f, "Type::U64"),
        }
    }
}
