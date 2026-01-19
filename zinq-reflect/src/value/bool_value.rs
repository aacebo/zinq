use crate::{
    ty::{BoolType, Type},
    value::{Value, ZinqValue},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BoolValue(bool);

impl BoolValue {
    pub fn as_bool(&self) -> &bool {
        &self.0
    }

    pub fn to_bool(&self) -> bool {
        self.0
    }
}

impl From<BoolValue> for Value {
    fn from(value: BoolValue) -> Self {
        Self::Bool(value)
    }
}

impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ZinqValue for BoolValue {
    fn ty(&self) -> Type {
        BoolType.into()
    }
}
