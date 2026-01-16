mod float_type;
mod int_type;
mod uint_type;

pub use float_type::*;
pub use int_type::*;
pub use uint_type::*;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Invalid,
    Bool,
    String,
    Int(IntType),
    UInt(UIntType),
    Float(FloatType),
}
