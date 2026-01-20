use zinq_types::{TypePtr, ZinqType, ty::FloatType};

use crate::value::{Value, ZinqValue};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum FloatValue {
    F32(f32),
    F64(f64),
}

impl FloatValue {
    pub fn is_f32(&self) -> bool {
        match self {
            Self::F32(_) => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Self::F64(_) => true,
            _ => false,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Self::F32(v) => *v,
            v => panic!("{}", format!("expected f32, received {}", v.ty())),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::F64(v) => *v,
            v => panic!("{}", format!("expected f64, received {}", v.ty())),
        }
    }
}

impl ZinqValue for FloatValue {
    fn ty(&self) -> TypePtr {
        match self {
            Self::F32(_) => FloatType::F32.ptr(),
            Self::F64(_) => FloatType::F64.ptr(),
        }
    }
}

impl From<FloatValue> for Value {
    fn from(value: FloatValue) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for FloatValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for FloatValue {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl std::fmt::Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F32(v) => write!(f, "{}", v),
            Self::F64(v) => write!(f, "{}", v),
        }
    }
}
