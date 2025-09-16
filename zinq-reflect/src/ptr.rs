use std::ops::{Deref, DerefMut};

use crate::{Reflect, TypeOf};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PtrType(Box<crate::Type>);

impl PtrType {
    pub fn id(&self) -> std::any::TypeId {
        return std::any::TypeId::of::<&Self>();
    }

    pub fn name(&self) -> &str {
        return stringify!(format!("&{}", self.0.name()));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Ptr(self.clone());
    }

    pub fn is_ptr_of(&self, _type: crate::Type) -> bool {
        return _type.eq(&self.0);
    }

    pub fn assignable_to(&self, _type: crate::Type) -> bool {
        return self.id() == _type.id();
    }

    pub fn convertable_to(&self, _type: crate::Type) -> bool {
        return _type.is_ptr_of(*self.0.clone());
    }
}

impl std::fmt::Display for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.name());
    }
}

impl<T> TypeOf for &T
where
    T: TypeOf,
{
    fn type_of() -> crate::Type {
        return T::type_of();
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ptr(Box<crate::Value>);

impl Ptr {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Ptr(PtrType(Box::new(self.0.to_type())));
    }

    pub fn get(&self) -> Box<crate::Value> {
        return self.0.clone();
    }
}

impl<T: Clone + Reflect> From<&T> for Ptr {
    fn from(value: &T) -> Self {
        return Self(Box::new(value.reflect()));
    }
}

impl std::fmt::Display for Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_ref());
    }
}

impl Reflect for Ptr {
    fn reflect(self) -> crate::Value {
        return crate::Value::Ptr(self.clone());
    }
}

impl<T> Reflect for &T
where
    T: Clone + Reflect,
{
    fn reflect(self) -> crate::Value {
        let value = self.clone();
        return crate::Value::Ptr(Ptr(Box::new(value.reflect())));
    }
}

impl AsRef<crate::Value> for Ptr {
    fn as_ref(&self) -> &crate::Value {
        return &self.0;
    }
}

impl AsMut<crate::Value> for Ptr {
    fn as_mut(&mut self) -> &mut crate::Value {
        return &mut self.0;
    }
}

impl Deref for Ptr {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Ptr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn int() {
        let value = value_of!(&3_i32);

        assert!(value.is_ptr());
        assert!(value.is_ptr_of(type_of!(i32)));
        assert_eq!(value.to_ptr().get().to_i32().get(), 3);
    }

    #[test]
    pub fn bool() {
        let value = value_of!(&true);

        assert!(value.is_ptr());
        assert!(value.is_ptr_of(type_of!(bool)));
        assert!(value.to_ptr().get().to_bool().get());
    }
}
