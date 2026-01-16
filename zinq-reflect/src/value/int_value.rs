use crate::value::UIntValue;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    UInt(UIntValue),
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
