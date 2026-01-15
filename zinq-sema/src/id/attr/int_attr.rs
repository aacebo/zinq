use crate::id::Attr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntAttr {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl From<u8> for IntAttr {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for IntAttr {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for IntAttr {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for IntAttr {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<u8> for Attr {
    fn from(value: u8) -> Self {
        Self::Int(value.into())
    }
}

impl From<u16> for Attr {
    fn from(value: u16) -> Self {
        Self::Int(value.into())
    }
}

impl From<u32> for Attr {
    fn from(value: u32) -> Self {
        Self::Int(value.into())
    }
}

impl From<u64> for Attr {
    fn from(value: u64) -> Self {
        Self::Int(value.into())
    }
}

impl std::fmt::Display for IntAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
        }
    }
}
