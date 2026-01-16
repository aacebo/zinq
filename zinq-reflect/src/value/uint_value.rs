#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIntValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
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
