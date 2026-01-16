#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatType {
    F32,
    F64,
}

impl std::fmt::Display for FloatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F32 => write!(f, "Type::F32"),
            Self::F64 => write!(f, "Type::F64"),
        }
    }
}
