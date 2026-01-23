mod bool_value;
mod float_value;
mod int_value;
mod uint_value;

pub use bool_value::*;
pub use float_value::*;
pub use int_value::*;
pub use uint_value::*;

use zinq_types::TypeId;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Bool(BoolValue),
    Float(FloatValue),
    Int(IntValue),
    UInt(UIntValue),
    Object(zinq_gc::GcAny),
}

impl ZinqValue for Value {
    fn ty(&self) -> TypeId {
        match self {
            Self::Bool(v) => v.ty(),
            Self::Float(v) => v.ty(),
            Self::Int(v) => v.ty(),
            Self::UInt(v) => v.ty(),
            Self::Object(v) => *v.header().ty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::UInt(v) => write!(f, "{}", v),
            Self::Object(v) => write!(f, "{:#?}", v),
        }
    }
}

pub trait ZinqValue {
    fn ty(&self) -> zinq_types::TypeId;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValueId(u32);

impl ValueId {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for ValueId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ValueId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ValueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
