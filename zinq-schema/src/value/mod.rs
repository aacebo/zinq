use std::fmt;

mod array;
mod number;
mod object;

pub use array::*;
pub use number::*;
pub use object::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(Object),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Null => write!(f, "null"),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "\"{}\"", v),
            Self::Array(v) => write!(f, "{}", v),
            Self::Object(v) => write!(f, "{}", v),
        };
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        return Self::Array(Array::from(value));
    }
}

impl From<&[Value]> for Value {
    fn from(value: &[Value]) -> Self {
        return Self::Array(Array::from(value));
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        return Self::Bool(value);
    }
}

impl From<&bool> for Value {
    fn from(value: &bool) -> Self {
        return Self::Bool(*value);
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        return Self::Number(Number::from(value));
    }
}

impl From<&i64> for Value {
    fn from(value: &i64) -> Self {
        return Self::Number(Number::from(*value));
    }
}
