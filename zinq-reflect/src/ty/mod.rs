mod array_type;
mod bool_type;
mod float_type;
mod int_type;
mod ptr_type;
mod string_type;
mod tuple_type;
mod uint_type;

pub use array_type::*;
pub use bool_type::*;
pub use float_type::*;
pub use int_type::*;
pub use ptr_type::*;
pub use string_type::*;
pub use tuple_type::*;
pub use uint_type::*;

use crate::{Path, Size};

pub trait ZinqType {
    fn path(&self) -> Path;
    fn size(&self) -> Size;
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool(BoolType),
    String(StringType),
    Int(IntType),
    UInt(UIntType),
    Float(FloatType),
    Ptr(PtrType),
    Tuple(TupleType),
}

impl ZinqType for Type {
    fn path(&self) -> Path {
        match self {
            Self::Bool(v) => v.path(),
            Self::String(v) => v.path(),
            Self::Int(v) => v.path(),
            Self::UInt(v) => v.path(),
            Self::Float(v) => v.path(),
            Self::Ptr(v) => v.path(),
            Self::Tuple(v) => v.path(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::Bool(v) => v.size(),
            Self::String(v) => v.size(),
            Self::Int(v) => v.size(),
            Self::UInt(v) => v.size(),
            Self::Float(v) => v.size(),
            Self::Ptr(v) => v.size(),
            Self::Tuple(v) => v.size(),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
