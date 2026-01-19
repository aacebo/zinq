mod bool_value;
mod float_value;
mod int_value;
mod uint_value;

pub use bool_value::*;
pub use float_value::*;
pub use int_value::*;
pub use uint_value::*;

use crate::ty::Type;

pub trait ZinqValue {
    fn ty(&self) -> Type;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(BoolValue),
    Float(FloatValue),
    Int(IntValue),
    UInt(UIntValue),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::UInt(v) => write!(f, "{}", v),
        }
    }
}
