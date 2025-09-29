use std::ops::{Deref, DerefMut};

use crate::ToValue;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefType(Box<crate::Type>);

impl RefType {
    pub fn new(ty: &crate::Type) -> Self {
        return Self(Box::new(ty.clone()));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Ref(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("&{}", self.0.id()));
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.0;
    }

    pub fn is_ref_of(&self, ty: crate::Type) -> bool {
        return ty.eq(&self.0);
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_ref_of(*self.0.clone());
    }
}

impl std::fmt::Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "&{}", &self.0);
    }
}

impl crate::ToType for RefType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Ref(self.clone());
    }
}

impl<T> crate::TypeOf for &T
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return crate::RefType::new(&T::type_of()).to_type();
    }
}

impl<T> crate::ToType for &T
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return crate::RefType::new(&T::type_of()).to_type();
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ref(Box<crate::Value>);

impl Ref {
    pub fn to_type(&self) -> crate::Type {
        return RefType::new(&self.0.to_type()).to_type();
    }

    pub fn get(&self) -> Box<crate::Value> {
        return self.0.clone();
    }

    pub fn get_mut(&mut self) -> &mut crate::Value {
        return &mut self.0;
    }

    pub fn set(&mut self, value: crate::Value) {
        self.0 = value.to_ref().get();
    }

    pub fn set_ref(&mut self, value: Box<crate::Value>) {
        self.0 = value;
    }
}

impl crate::Value {
    pub fn set_ref(&mut self, value: Box<crate::Value>) {
        return match self {
            Self::Ref(v) => v.set_ref(value),
            _ => panic!("called 'set_ref' on '{}'", self.to_type()),
        };
    }
}

impl<T: Clone + crate::ToValue> From<&T> for Ref {
    fn from(value: &T) -> Self {
        return Self(Box::new(value.to_value()));
    }
}

impl std::fmt::Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_ref());
    }
}

impl crate::ToType for Ref {
    fn to_type(&self) -> crate::Type {
        return RefType::new(&self.0.to_type()).to_type();
    }
}

impl crate::ToValue for Ref {
    fn to_value(self) -> crate::Value {
        return crate::Value::Ref(self.clone());
    }
}

impl<T> crate::ToValue for &T
where
    T: Clone + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        let value = self.clone();
        return crate::Value::Ref(Ref(Box::new(value.to_value())));
    }
}

impl AsRef<crate::Value> for Ref {
    fn as_ref(&self) -> &crate::Value {
        return &self.0;
    }
}

impl AsMut<crate::Value> for Ref {
    fn as_mut(&mut self) -> &mut crate::Value {
        return &mut self.0;
    }
}

impl Deref for Ref {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Ref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Eq for Ref {}

impl Ord for Ref {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.0.cmp(&other.0);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn int() {
        let value = value_of!(&3_i32);

        assert!(value.is_ref());
        assert!(value.is_ref_of(type_of!(i32)));
        assert_eq!(value.to_type().id(), "&i32");
        assert_eq!(value.to_ref().get().to_i32().get(), 3);
    }

    #[test]
    pub fn bool() {
        let value = value_of!(&true);

        assert!(value.is_ref());
        assert!(value.is_ref_of(type_of!(bool)));
        assert_eq!(value.to_type().id(), "&bool");
        assert!(value.to_ref().get().to_bool().get());
    }
}
