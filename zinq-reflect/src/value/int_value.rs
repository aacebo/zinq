use crate::{
    ty::{IntType, Type, ZinqType},
    value::{Value, ZinqValue},
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum IntValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl IntValue {
    pub fn is_i8(&self) -> bool {
        match self {
            Self::I8(_) => true,
            _ => false,
        }
    }

    pub fn is_i16(&self) -> bool {
        match self {
            Self::I16(_) => true,
            _ => false,
        }
    }

    pub fn is_i32(&self) -> bool {
        match self {
            Self::I32(_) => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Self::I64(_) => true,
            _ => false,
        }
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Self::I8(v) => *v,
            v => panic!("{}", format!("expected i8, received {}", v.ty().name())),
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Self::I16(v) => *v,
            v => panic!("{}", format!("expected i16, received {}", v.ty().name())),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::I32(v) => *v,
            v => panic!("{}", format!("expected i32, received {}", v.ty().name())),
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::I64(v) => *v,
            v => panic!("{}", format!("expected i64, received {}", v.ty().name())),
        }
    }
}

impl ZinqValue for IntValue {
    fn ty(&self) -> Type {
        match self {
            Self::I8(_) => IntType::I8.into(),
            Self::I16(_) => IntType::I16.into(),
            Self::I32(_) => IntType::I32.into(),
            Self::I64(_) => IntType::I64.into(),
        }
    }
}

impl From<IntValue> for Value {
    fn from(value: IntValue) -> Self {
        Self::Int(value)
    }
}

impl From<i8> for IntValue {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for IntValue {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for IntValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for IntValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl std::fmt::Display for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(v) => write!(f, "{}", v),
            Self::I16(v) => write!(f, "{}", v),
            Self::I32(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
        }
    }
}
