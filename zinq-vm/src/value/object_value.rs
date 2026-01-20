use zinq_types::TypePtr;

use crate::value::{StringValue, Value, ZinqValue};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectValue {
    String(StringValue),
}

impl From<ObjectValue> for Value {
    fn from(value: ObjectValue) -> Self {
        Self::Object(value)
    }
}

impl ZinqValue for ObjectValue {
    fn ty(&self) -> TypePtr {
        match self {
            Self::String(v) => v.ty(),
        }
    }
}

impl std::fmt::Display for ObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
        }
    }
}
