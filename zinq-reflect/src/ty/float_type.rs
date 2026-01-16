use crate::{Path, Size, ty::ZinqType};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatType {
    F32,
    F64,
}

impl ZinqType for FloatType {
    fn path(&self) -> Path {
        match self {
            Self::F32 => "f32".into(),
            Self::F64 => "f64".into(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::F32 => Size::Static(4),
            Self::F64 => Size::Static(8),
        }
    }
}

impl std::fmt::Display for FloatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
