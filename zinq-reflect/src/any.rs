use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Any {
    pub(crate) ty: crate::Type,
    pub(crate) value: Arc<dyn std::any::Any>,
}

impl Any {
    pub fn new<T: std::any::Any + crate::ToType>(value: T) -> Self {
        return Self {
            ty: value.to_type(),
            value: Arc::new(value),
        };
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }

    pub fn get(&self) -> &dyn std::any::Any {
        return &self.value;
    }

    pub fn get_mut(&mut self) -> &mut dyn std::any::Any {
        return &mut self.value;
    }

    pub fn set<T: std::any::Any>(&mut self, value: T) {
        self.value = Arc::new(value);
    }

    pub fn is<T: std::any::Any>(&self) -> bool {
        return self.value.is::<T>();
    }

    pub fn to<T: std::any::Any + crate::TypeOf>(&self) -> &T {
        if let Some(v) = self.value.downcast_ref::<T>() {
            return v;
        }

        panic!("failed to cast value to type '{}'", T::type_of());
    }

    pub fn to_mut<T: std::any::Any>(&mut self) -> &mut T {
        return Arc::get_mut(&mut self.value)
            .unwrap()
            .downcast_mut::<T>()
            .unwrap();
    }
}

impl crate::TypeOf for Any {
    fn type_of() -> crate::Type {
        return crate::Type::Any;
    }
}

impl crate::ToType for Any {
    fn to_type(&self) -> crate::Type {
        return self.ty.clone();
    }
}

impl crate::ToValue for Any {
    fn to_value(self) -> crate::Value {
        return crate::Value::Any(self.clone());
    }
}

impl AsRef<dyn std::any::Any> for Any {
    fn as_ref(&self) -> &dyn std::any::Any {
        return &self.value;
    }
}

impl AsMut<dyn std::any::Any> for Any {
    fn as_mut(&mut self) -> &mut dyn std::any::Any {
        return &mut self.value;
    }
}

impl std::ops::Deref for Any {
    type Target = dyn std::any::Any;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl std::ops::DerefMut for Any {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.value;
    }
}

impl std::ops::Index<usize> for Any {
    type Output = dyn std::any::Any;

    fn index(&self, _index: usize) -> &Self::Output {
        if self.ty.is_slice() {
            return &self.value;
        }

        panic!("called 'std::ops::Index<usize>' on {}", self.ty);
    }
}

impl std::fmt::Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self.value.downcast_ref::<bool>() {
            return write!(f, "{}", v);
        }

        return write!(f, "{:?}", &self.value);
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.ty.is_bool() {
            return serializer.serialize_bool(*self.value.downcast_ref::<bool>().unwrap());
        }

        return serializer.serialize_str("null");
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Any {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
