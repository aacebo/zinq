#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelfType;

impl SelfType {
    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str("Self");
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::_Self(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_self();
    }
}

impl std::fmt::Display for SelfType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}
