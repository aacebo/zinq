use std::ops::{Deref, DerefMut, Index};

use crate::{Reflect, TypeOf};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringType;

impl StringType {
    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str("string");
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::String(self.clone());
    }

    pub fn assignable_to(&self, _type: crate::Type) -> bool {
        return self.id() == _type.id();
    }

    pub fn convertable_to(&self, _type: crate::Type) -> bool {
        return _type.is_string();
    }
}

impl std::fmt::Display for StringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl TypeOf for std::string::String {
    fn type_of() -> crate::Type {
        return crate::Type::String(StringType::default());
    }
}

impl TypeOf for str {
    fn type_of() -> crate::Type {
        return crate::Type::String(StringType::default());
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String(std::string::String);

impl String {
    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }

    pub fn get(&self) -> std::string::String {
        return self.0.clone();
    }
}

impl From<std::string::String> for crate::Value {
    fn from(value: std::string::String) -> Self {
        return Self::String(String(value));
    }
}

impl Into<std::string::String> for crate::Value {
    fn into(self) -> std::string::String {
        return self.to_string().get();
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        return Self(value);
    }
}

impl Into<std::string::String> for String {
    fn into(self) -> std::string::String {
        return self.0;
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_ref());
    }
}

impl TypeOf for String {
    fn type_of() -> crate::Type {
        return std::string::String::type_of();
    }
}

impl Reflect for String {
    fn reflect(self) -> crate::Value {
        return crate::Value::String(self.clone());
    }
}

impl Reflect for std::string::String {
    fn reflect(self) -> crate::Value {
        return crate::Value::String(String(self.clone()));
    }
}

impl Reflect for &str {
    fn reflect(self) -> crate::Value {
        return crate::Value::String(String(self.to_string()));
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl AsMut<std::string::String> for String {
    fn as_mut(&mut self) -> &mut std::string::String {
        return &mut self.0;
    }
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Index<usize> for String {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return self.as_bytes().index(index);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn str() {
        let value = value_of!("test");

        assert!(value.is_string());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string().as_ref(), "test");
    }

    #[test]
    pub fn string() {
        let value = value_of!("test".to_string());

        assert!(value.is_string());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string().as_ref(), "test");
    }
}
