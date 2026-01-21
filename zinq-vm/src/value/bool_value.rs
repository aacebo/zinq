use zinq_types::{TypeId, ZinqType, ty::BoolType};

use crate::value::{Value, ZinqValue};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BoolValue(bool);

impl BoolValue {
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

impl std::ops::Deref for BoolValue {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ZinqValue for BoolValue {
    fn ty(&self) -> TypeId {
        BoolType.id()
    }
}
