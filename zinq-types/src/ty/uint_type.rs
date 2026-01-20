use crate::{Size, ZinqType, ty::Type};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIntType {
    U8,
    U16,
    U32,
    U64,
}

impl UIntType {
    pub fn is_u8(&self) -> bool {
        match self {
            Self::U8 => true,
            _ => false,
        }
    }

    pub fn is_u16(&self) -> bool {
        match self {
            Self::U16 => true,
            _ => false,
        }
    }

    pub fn is_u32(&self) -> bool {
        match self {
            Self::U32 => true,
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match self {
            Self::U64 => true,
            _ => false,
        }
    }
}

impl ZinqType for UIntType {
    fn name(&self) -> String {
        match self {
            Self::U8 => "u8".into(),
            Self::U16 => "u16".into(),
            Self::U32 => "u32".into(),
            Self::U64 => "u64".into(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::U8 => Size::Known(1),
            Self::U16 => Size::Known(2),
            Self::U32 => Size::Known(4),
            Self::U64 => Size::Known(8),
        }
    }
}

impl From<UIntType> for Type {
    fn from(value: UIntType) -> Self {
        Self::UInt(value)
    }
}

impl std::fmt::Display for UIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
