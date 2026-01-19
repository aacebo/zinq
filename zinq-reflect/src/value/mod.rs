mod bool_value;
mod float_value;
mod int_value;
mod string_value;
mod tuple_value;
mod uint_value;

pub use bool_value::*;
pub use float_value::*;
pub use int_value::*;
pub use string_value::*;
pub use tuple_value::*;
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
    Tuple(TupleValue),
    String(StringValue),
}

impl ZinqValue for Value {
    fn ty(&self) -> Type {
        match self {
            Self::Bool(v) => v.ty(),
            Self::Float(v) => v.ty(),
            Self::Int(v) => v.ty(),
            Self::UInt(v) => v.ty(),
            Self::Tuple(v) => v.ty(),
            Self::String(v) => v.ty(),
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
            Self::Tuple(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
        }
    }
}
