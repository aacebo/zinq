use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{Reflect, TypeOf};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedSliceType {
    _type: Box<crate::Type>,
    size: usize,
}

impl SizedSliceType {
    pub fn id(&self) -> std::any::TypeId {
        return std::any::TypeId::of::<&Self>();
    }

    pub fn name(&self) -> &str {
        return stringify!(format!("[{}; {}]", &self._type.name(), &self.size));
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::Sized(self.clone()));
    }

    pub fn is_slice_of(&self, _type: crate::Type) -> bool {
        return _type.eq(&self._type);
    }

    pub fn assignable_to(&self, _type: crate::Type) -> bool {
        return self.id() == _type.id();
    }

    pub fn convertable_to(&self, _type: crate::Type) -> bool {
        return _type.is_slice_of(*self._type.clone());
    }
}

impl std::fmt::Display for SizedSliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.name());
    }
}

impl<const N: usize, T> TypeOf for [T; N]
where
    T: TypeOf,
{
    fn type_of() -> crate::Type {
        return crate::Type::Slice(crate::SliceType::Sized(SizedSliceType {
            _type: Box::new(T::type_of()),
            size: N,
        }));
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedSlice {
    _type: crate::Type,
    size: usize,
    value: Vec<crate::Value>,
}

impl SizedSlice {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(crate::SliceType::Sized(SizedSliceType {
            _type: Box::new(self._type.clone()),
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
            _type: value.first().unwrap().to_type(),
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
            _type: T::type_of(),
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

        for (i, value) in value.to_slice().iter().enumerate() {
            assert!(value.is_i32());
            assert_eq!(i + 1, value.to_i32().get() as usize);
        }
    }
}
