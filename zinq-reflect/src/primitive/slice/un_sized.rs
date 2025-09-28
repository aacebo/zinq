use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::TypeOf;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnSizedSliceType {
    ty: Box<crate::Type>,
}

impl UnSizedSliceType {
    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("[{}]", self.ty.id()));
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

impl std::fmt::Display for UnSizedSliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl<T> crate::TypeOf for [T]
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return crate::Type::Slice(crate::SliceType::UnSized(UnSizedSliceType {
            ty: Box::new(T::type_of()),
        }));
    }
}

impl crate::ToType for UnSizedSliceType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::UnSized(self.clone()));
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnSizedSlice {
    ty: crate::Type,
    value: Vec<crate::Value>,
}

impl UnSizedSlice {
    pub fn len(&self) -> usize {
        return self.value.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return self.value.iter();
    }

    pub fn get(&self) -> Vec<crate::Value> {
        return self.value.clone();
    }
}

impl From<&[crate::Value]> for UnSizedSlice {
    fn from(value: &[crate::Value]) -> Self {
        return Self {
            ty: value.first().unwrap().to_type(),
            value: value.to_vec(),
        };
    }
}

impl std::fmt::Display for UnSizedSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in &self.value {
            write!(f, "{}", value)?;
        }

        return Ok(());
    }
}

impl crate::ToType for UnSizedSlice {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::UnSized(UnSizedSliceType {
            ty: Box::new(self.ty.clone()),
        }));
    }
}

impl crate::ToValue for UnSizedSlice {
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(crate::Slice::UnSized(self.clone()));
    }
}

impl<T> crate::ToValue for &[T]
where
    T: Clone + TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Slice(crate::Slice::UnSized(UnSizedSlice {
            ty: T::type_of(),
            value: self.iter().map(|v| v.clone().to_value()).collect(),
        }));
    }
}

impl AsRef<[crate::Value]> for UnSizedSlice {
    fn as_ref(&self) -> &[crate::Value] {
        return self.value.as_slice();
    }
}

impl AsMut<[crate::Value]> for UnSizedSlice {
    fn as_mut(&mut self) -> &mut [crate::Value] {
        return self.value.as_mut_slice();
    }
}

impl Deref for UnSizedSlice {
    type Target = [crate::Value];

    fn deref(&self) -> &Self::Target {
        return self.value.as_slice();
    }
}

impl DerefMut for UnSizedSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.value.as_mut_slice();
    }
}

impl Index<usize> for UnSizedSlice {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        return self.value.index(index);
    }
}

impl IndexMut<usize> for UnSizedSlice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.value.index_mut(index);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn un_sized() {
        let value = value_of!(vec![1, 2, 3].as_slice());

        assert!(value.is_slice());
        assert!(value.to_slice().is_unsized());
        assert_eq!(value.len(), 3);
        assert_eq!(value.to_type().id(), "[i32]");

        for (i, value) in value.to_slice().iter().enumerate() {
            assert!(value.is_i32());
            assert_eq!(i + 1, value.to_i32().get() as usize);
        }
    }
}
