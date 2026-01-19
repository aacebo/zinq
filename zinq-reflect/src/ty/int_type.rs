use crate::{
    Size,
    ty::{Type, ZinqType},
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
}

impl IntType {
    pub fn is_i8(&self) -> bool {
        match self {
            Self::I8 => true,
            _ => false,
        }
    }

    pub fn is_i16(&self) -> bool {
        match self {
            Self::I16 => true,
            _ => false,
        }
    }

    pub fn is_i32(&self) -> bool {
        match self {
            Self::I32 => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Self::I64 => true,
            _ => false,
        }
    }
}

impl ZinqType for IntType {
    fn name(&self) -> String {
        match self {
            Self::I8 => "i8".into(),
            Self::I16 => "i16".into(),
            Self::I32 => "i32".into(),
            Self::I64 => "i64".into(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::I8 => Size::Static(1),
            Self::I16 => Size::Static(2),
            Self::I32 => Size::Static(4),
            Self::I64 => Size::Static(8),
        }
    }
}

impl From<IntType> for Type {
    fn from(value: IntType) -> Self {
        Self::Int(value)
    }
}

impl std::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
