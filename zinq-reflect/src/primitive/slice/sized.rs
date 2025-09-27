use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{Reflect, TypeOf};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedSliceType {
    ty: Box<crate::Type>,
    size: usize,
}

impl SizedSliceType {
    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("[{}; {}]", &self.ty.id(), &self.size));
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::Sized(self.clone()));
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

impl std::fmt::Display for SizedSliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl<const N: usize, T> TypeOf for [T; N]
where
    T: TypeOf,
{
    fn type_of() -> crate::Type {
        let ty = T::type_of();

        return crate::Type::Slice(crate::SliceType::Sized(SizedSliceType {
            ty: Box::new(ty),
            size: N,
        }));
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedSlice {
    ty: crate::Type,
    size: usize,
    value: Vec<crate::Value>,
}

impl SizedSlice {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::Sized(SizedSliceType {
            ty: Box::new(self.ty.clone()),
            size: self.size,
        }));
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return self.value.iter();
    }

    pub fn get(&self) -> Vec<crate::Value> {
        return self.value.clone();
    }
}

impl<const N: usize> From<[crate::Value; N]> for SizedSlice {
    fn from(value: [crate::Value; N]) -> Self {
        return Self {
            ty: value.first().unwrap().to_type(),
            size: N,
            value: value.to_vec(),
        };
    }
}

impl std::fmt::Display for SizedSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in &self.value {
            write!(f, "{}", value)?;
        }

        return Ok(());
    }
}

impl Reflect for SizedSlice {
    fn reflect(self) -> crate::Value {
        return crate::Value::Slice(crate::Slice::Sized(self.clone()));
    }
}

impl<const N: usize, T> Reflect for [T; N]
where
    T: Clone + TypeOf + Reflect,
{
    fn reflect(self) -> crate::Value {
        return crate::Value::Slice(crate::Slice::Sized(SizedSlice {
            ty: T::type_of(),
            size: N,
            value: self.iter().map(|v| v.clone().reflect()).collect(),
        }));
    }
}

impl AsRef<[crate::Value]> for SizedSlice {
    fn as_ref(&self) -> &[crate::Value] {
        return self.value.as_slice();
    }
}

impl AsMut<[crate::Value]> for SizedSlice {
    fn as_mut(&mut self) -> &mut [crate::Value] {
        return self.value.as_mut_slice();
    }
}

impl Deref for SizedSlice {
    type Target = [crate::Value];

    fn deref(&self) -> &Self::Target {
        return self.value.as_slice();
    }
}

impl DerefMut for SizedSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self.value.as_mut_slice();
    }
}

impl Index<usize> for SizedSlice {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        return self.value.index(index);
    }
}

impl IndexMut<usize> for SizedSlice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.value.index_mut(index);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn sized() {
        let value = value_of!([1, 2, 3]);

        assert!(value.is_slice());
        assert!(value.to_slice().is_sized());
        assert_eq!(value.len(), 3);
        assert_eq!(value.to_type().len(), 3);
        assert_eq!(value.to_type().id(), "[i32; 3]");

        for (i, value) in value.to_slice().iter().enumerate() {
            assert!(value.is_i32());
            assert_eq!(i + 1, value.to_i32().get() as usize);
        }
    }
}
