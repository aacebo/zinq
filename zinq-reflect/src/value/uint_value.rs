use crate::{
    ty::{Type, UIntType, ZinqType},
    value::{Value, ZinqValue},
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum UIntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl UIntValue {
    pub fn is_u8(&self) -> bool {
        match self {
            Self::U8(_) => true,
            _ => false,
        }
    }

    pub fn is_u16(&self) -> bool {
        match self {
            Self::U16(_) => true,
            _ => false,
        }
    }

    pub fn is_u32(&self) -> bool {
        match self {
            Self::U32(_) => true,
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match self {
            Self::U64(_) => true,
            _ => false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::U8(v) => *v,
            v => panic!("{}", format!("expected u8, received {}", v.ty().name())),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::U16(v) => *v,
            v => panic!("{}", format!("expected u16, received {}", v.ty().name())),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::U32(v) => *v,
            v => panic!("{}", format!("expected u32, received {}", v.ty().name())),
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Self::U64(v) => *v,
            v => panic!("{}", format!("expected u64, received {}", v.ty().name())),
        }
    }
}

impl ZinqValue for UIntValue {
    fn ty(&self) -> Type {
        match self {
            Self::U8(_) => UIntType::U8.into(),
            Self::U16(_) => UIntType::U16.into(),
            Self::U32(_) => UIntType::U32.into(),
            Self::U64(_) => UIntType::U64.into(),
        }
    }
}

impl From<UIntValue> for Value {
    fn from(value: UIntValue) -> Self {
        Self::UInt(value)
    }
}

impl From<u8> for UIntValue {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for UIntValue {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for UIntValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for UIntValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl std::fmt::Display for UIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
        }
    }
}
