use std::ops::{Deref, DerefMut};

use crate::TypeOf;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolType;

impl BoolType {
    pub fn to_type(&self) -> crate::Type {
        return bool::type_of();
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str("bool");
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_bool();
    }
}

impl std::fmt::Display for BoolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl crate::ToType for BoolType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Bool(self.clone());
    }
}

impl crate::TypeOf for bool {
    fn type_of() -> crate::Type {
        return crate::Type::Bool(BoolType::default());
    }
}

impl crate::ToType for bool {
    fn to_type(&self) -> crate::Type {
        return Self::type_of();
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
pub struct Bool(bool);

impl Bool {
    pub fn is_true(&self) -> bool {
        return self.0 == true;
    }

    pub fn is_false(&self) -> bool {
        return self.0 == false;
    }

    pub fn get(&self) -> bool {
        return self.0;
    }

    pub fn set(&mut self, value: crate::Value) {
        self.0 = value.to_bool().get();
    }

    pub fn set_bool(&mut self, value: bool) {
        self.0 = value;
    }
}

impl crate::Value {
    pub fn set_bool(&mut self, value: bool) {
        return match self {
            Self::Bool(v) => v.set_bool(value),
            _ => panic!("called 'set_bool' on '{}'", self.to_type()),
        };
    }
}

impl From<bool> for crate::Value {
    fn from(value: bool) -> Self {
        return Self::Bool(Bool(value));
    }
}

impl Into<bool> for crate::Value {
    fn into(self) -> bool {
        return self.to_bool().get();
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        return Self(value);
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        return self.0;
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_ref());
    }
}

impl crate::TypeOf for Bool {
    fn type_of() -> crate::Type {
        return bool::type_of();
    }
}

impl crate::ToType for Bool {
    fn to_type(&self) -> crate::Type {
        return bool::type_of();
    }
}

impl crate::ToValue for Bool {
    fn to_value(self) -> crate::Value {
        return crate::Value::Bool(self.clone());
    }
}

impl crate::ToValue for bool {
    fn to_value(self) -> crate::Value {
        return crate::Value::Bool(Bool(self.clone()));
    }
}

impl AsRef<bool> for Bool {
    fn as_ref(&self) -> &bool {
        return &self.0;
    }
}

impl AsMut<bool> for Bool {
    fn as_mut(&mut self) -> &mut bool {
        return &mut self.0;
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Eq for Bool {}

impl Ord for Bool {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.0.cmp(&other.0);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn truthy() {
        let value = value_of!(true);

        assert!(value.is_bool());
        assert!(value.is_true());
        assert!(value.to_bool().get());
    }

    #[test]
    pub fn falsy() {
        let value = value_of!(false);

        assert!(value.is_bool());
        assert!(value.is_false());
        assert!(!value.to_bool().get());
    }
}
