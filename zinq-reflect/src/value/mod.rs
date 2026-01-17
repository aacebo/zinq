mod int_value;
mod uint_value;

pub use int_value::*;
pub use uint_value::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(IntValue),
    UInt(UIntValue),
}
