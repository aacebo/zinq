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

impl PartialEq<crate::Type> for BoolType {
    fn eq(&self, other: &crate::Type) -> bool {
        return other.is_bool() && other.as_bool() == self;
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

impl From<bool> for crate::Value {
    fn from(value: bool) -> Self {
        return Self::Bool(value);
    }
}

impl crate::ToValue for bool {
    fn to_value(self) -> crate::Value {
        return crate::Value::Bool(self);
    }
}

impl crate::Value {
    pub fn is_true(&self) -> bool {
        return self.is_bool() && self.to_bool() == true;
    }

    pub fn is_false(&self) -> bool {
        return self.is_bool() && self.to_bool() == false;
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
        assert!(value.to_bool());
    }

    #[test]
    pub fn falsy() {
        let value = value_of!(false);

        assert!(value.is_bool());
        assert!(value.is_false());
        assert!(!value.to_bool());
    }
}
