use crate::{Path, Size, ty::ZinqType};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
}

impl ZinqType for IntType {
    fn path(&self) -> Path {
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

impl std::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
