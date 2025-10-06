use std::sync::Arc;

impl dyn crate::Object {
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        let value: &dyn std::any::Any = self;
        return value.downcast_ref::<T>();
    }

    pub fn downcast_mut<T: std::any::Any>(&mut self) -> Option<&mut T> {
        let value: &mut dyn std::any::Any = self;
        return value.downcast_mut::<T>();
    }

    pub fn is<T: std::any::Any>(&self) -> bool {
        let value: &dyn std::any::Any = self;
        return value.is::<T>();
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn crate::Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let ty = self.to_type().to_struct();
        let mut ser = serializer.serialize_map(Some(ty.len()))?;

        for field in ty.fields().iter() {
            ser.serialize_entry(&field.name().to_string(), &self.field(field.name()))?;
        }

        return ser.end();
    }
}

#[derive(Debug, Clone)]
pub struct Any {
    pub(crate) ty: crate::Type,
    pub(crate) value: Arc<dyn crate::Object>,
}

impl Any {
    pub fn new<T: crate::Object>(value: T) -> Self {
        return Self {
            ty: value.to_type(),
            value: Arc::new(value),
        };
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }

    pub fn get(&self) -> &dyn crate::Object {
        return self.value.as_ref();
    }

    pub fn get_mut(&mut self) -> &mut dyn crate::Object {
        return Arc::get_mut(&mut self.value).unwrap();
    }

    pub fn set<T: crate::Object>(&mut self, value: T) {
        self.value = Arc::new(value);
    }

    pub fn is<T: crate::Object>(&self) -> bool {
        return self.as_ref().is::<T>();
    }

    pub fn to<T: crate::Object + crate::TypeOf>(&self) -> &T {
        if let Some(v) = self.as_ref().downcast_ref::<T>() {
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

impl AsRef<dyn crate::Object> for Any {
    fn as_ref(&self) -> &dyn crate::Object {
        return self.get();
    }
}

impl AsMut<dyn crate::Object> for Any {
    fn as_mut(&mut self) -> &mut dyn crate::Object {
        return self.get_mut();
    }
}

impl std::ops::Deref for Any {
    type Target = dyn crate::Object;

    fn deref(&self) -> &Self::Target {
        return self.get();
    }
}

impl std::ops::DerefMut for Any {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.get_mut();
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
        if self.ty.is_struct() {
            use serde::ser::SerializeMap;

            let mut ser = serializer.serialize_map(Some(self.ty.as_struct().len()))?;

            for field in self.ty.to_struct().fields().iter() {
                ser.serialize_entry(&field.name().to_string(), self.value.as_ref())?;
            }

            return ser.end();
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

impl<T: Clone + crate::Object> crate::Object for Arc<T> {
    fn field(&self, name: &crate::FieldName) -> crate::Value {
        return self.as_ref().field(name);
    }
}
