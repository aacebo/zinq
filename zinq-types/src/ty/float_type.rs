use crate::{Size, ZinqType, ty::Type};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatType {
    F32,
    F64,
}

impl FloatType {
    pub fn is_f32(&self) -> bool {
        match self {
            Self::F32 => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Self::F64 => true,
            _ => false,
        }
    }
}

impl ZinqType for FloatType {
    fn name(&self) -> String {
        match self {
            Self::F32 => "f32".into(),
            Self::F64 => "f64".into(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::F32 => Size::Known(4),
            Self::F64 => Size::Known(8),
        }
    }
}

impl From<FloatType> for Type {
    fn from(value: FloatType) -> Self {
        Self::Float(value)
    }
}

impl std::fmt::Display for FloatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
