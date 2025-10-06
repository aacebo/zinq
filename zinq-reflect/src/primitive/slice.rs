#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SliceType {
    ty: Box<crate::Type>,
    capacity: Option<usize>,
}

impl SliceType {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return match self.capacity {
            None => crate::TypeId::from_string(format!("[{}]", &self.ty.id())),
            Some(capacity) => {
                crate::TypeId::from_string(format!("[{}; {}]", &self.ty.id(), capacity))
            }
        };
    }

    pub fn len(&self) -> usize {
        return match self.capacity {
            None => panic!("called 'len' on unbound slice type"),
            Some(capacity) => capacity,
        };
    }

    pub fn capacity(&self) -> Option<usize> {
        return self.capacity;
    }

    pub fn is_slice_of(&self, ty: crate::Type) -> bool {
        return ty.eq(&self.ty);
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_slice_of(*self.ty.clone());
    }
}

impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl crate::ToType for SliceType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.clone());
    }
}

impl<const N: usize, T> crate::TypeOf for [T; N]
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: Some(N),
        });
    }
}

impl<T> crate::TypeOf for [T]
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: None,
        });
    }
}

impl<T> crate::TypeOf for Vec<T>
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: None,
        });
    }
}

impl<const N: usize, T> crate::ToType for [T; N]
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: Some(N),
        });
    }
}

impl<T> crate::ToType for [T]
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: None,
        });
    }
}

impl<T> crate::ToType for Vec<T>
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(SliceType {
            ty: Box::new(ty),
            capacity: None,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slice {
    pub(crate) ty: SliceType,
    pub(crate) value: Vec<crate::Value>,
}

impl Slice {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.ty.clone());
    }

    pub fn len(&self) -> usize {
        return self.value.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return self.value.iter();
    }
}

impl PartialEq<crate::Value> for Slice {
    fn eq(&self, other: &crate::Value) -> bool {
        return other.is_slice() && other.as_slice() == self;
    }
}

impl From<&[crate::Value]> for Slice {
    fn from(value: &[crate::Value]) -> Self {
        return Self {
            ty: SliceType {
                ty: Box::new(value.first().unwrap().to_type()),
                capacity: None,
            },
            value: value.to_vec(),
        };
    }
}

impl std::fmt::Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.value.iter().enumerate() {
            write!(f, "{}", value)?;

            if i < self.value.len() - 1 {
                write!(f, ", ")?;
            }
        }

        return write!(f, "]");
    }
}

impl crate::ToType for Slice {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.ty.clone());
    }
}

impl crate::ToValue for Slice {
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(self.clone());
    }
}

impl<T> crate::ToValue for &[T]
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(Slice {
            ty: SliceType {
                ty: Box::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.clone().to_value()).collect(),
        });
    }
}

impl<const N: usize, T> crate::ToValue for [T; N]
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(Slice {
            ty: SliceType {
                ty: Box::new(T::type_of()),
                capacity: Some(N),
            },
            value: self.iter().map(|v| v.clone().to_value()).collect(),
        });
    }
}

impl<T> crate::ToValue for Vec<T>
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(Slice {
            ty: SliceType {
                ty: Box::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.clone().to_value()).collect(),
        });
    }
}

impl AsRef<[crate::Value]> for Slice {
    fn as_ref(&self) -> &[crate::Value] {
        return self.value.as_slice();
    }
}

impl AsMut<[crate::Value]> for Slice {
    fn as_mut(&mut self) -> &mut [crate::Value] {
        return self.value.as_mut_slice();
    }
}

impl std::ops::Deref for Slice {
    type Target = [crate::Value];

    fn deref(&self) -> &Self::Target {
        return self.value.as_slice();
    }
}

impl std::ops::DerefMut for Slice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.value.as_mut_slice();
    }
}

impl std::ops::Index<usize> for Slice {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        return self.value.index(index);
    }
}

impl std::ops::IndexMut<usize> for Slice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.value.index_mut(index);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn ok() {
        let value = value_of!([1, 2, 3]);

        assert!(value.is_slice());
        assert_eq!(value.len(), 3);
        assert_eq!(value.to_type().len(), 3);
        assert_eq!(value.to_type().id(), "[i32; 3]");

        for (i, value) in value.as_slice().iter().enumerate() {
            assert!(value.is_i32());
            assert_eq!(i + 1, value.to_i32() as usize);
        }
    }
}
