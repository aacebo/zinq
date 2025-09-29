#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MutType(pub(crate) Box<crate::Type>);

impl MutType {
    pub fn new(ty: &crate::Type) -> Self {
        return Self(Box::new(ty.clone()));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Mut(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("mut {}", &self.0.id()));
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.0;
    }

    pub fn is_mut_of(&self, ty: crate::Type) -> bool {
        return ty.eq(&self.0);
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_mut();
    }
}

impl crate::ToType for MutType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Mut(self.clone());
    }
}

impl std::fmt::Display for MutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "mut {}", &self.0);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mut(Box<crate::Value>);

impl Mut {
    pub fn to_type(&self) -> crate::Type {
        return MutType::new(&self.0.to_type()).to_type();
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

    pub fn set_mut(&mut self, value: Box<crate::Value>) {
        self.0 = value;
    }
}

impl crate::Value {
    pub fn set_mut(&mut self, value: Box<crate::Value>) {
        return match self {
            Self::Mut(v) => v.set_mut(value),
            Self::Ref(v) => v.get().set_mut(value),
            _ => panic!("called 'set_mut' on '{}'", self.to_type()),
        };
    }
}

impl<T: Clone + crate::ToValue> From<&T> for Mut {
    fn from(value: &T) -> Self {
        return Self(Box::new(value.clone().to_value()));
    }
}

impl std::fmt::Display for Mut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_ref());
    }
}

impl crate::ToType for Mut {
    fn to_type(&self) -> crate::Type {
        return MutType::new(&self.0.to_type()).to_type();
    }
}

impl crate::ToValue for Mut {
    fn to_value(self) -> crate::Value {
        return crate::Value::Mut(self.clone());
    }
}

impl AsRef<crate::Value> for Mut {
    fn as_ref(&self) -> &crate::Value {
        return &self.0;
    }
}

impl AsMut<crate::Value> for Mut {
    fn as_mut(&mut self) -> &mut crate::Value {
        return &mut self.0;
    }
}

impl std::ops::Deref for Mut {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl std::ops::DerefMut for Mut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Eq for Mut {}

impl Ord for Mut {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.0.cmp(&other.0);
    }
}
