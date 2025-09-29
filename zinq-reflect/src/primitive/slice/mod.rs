pub mod sized;
pub mod un_sized;

use std::ops::{Deref, DerefMut, Index, IndexMut};

pub use sized::*;
pub use un_sized::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SliceType {
    Sized(SizedSliceType),
    UnSized(UnSizedSliceType),
}

impl SliceType {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Sized(v) => v.to_type(),
            Self::UnSized(v) => v.to_type(),
        };
    }

    pub fn id(&self) -> crate::TypeId {
        return match self {
            Self::Sized(v) => v.id(),
            Self::UnSized(v) => v.id(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Sized(v) => v.len(),
            Self::UnSized(_) => panic!("called 'len' on type '{}'", self.id()),
        };
    }

    pub fn is_sized(&self) -> bool {
        return match self {
            Self::Sized(_) => true,
            _ => false,
        };
    }

    pub fn is_unsized(&self) -> bool {
        return match self {
            Self::UnSized(_) => true,
            _ => false,
        };
    }

    pub fn is_slice_of(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.is_slice_of(ty),
            Self::UnSized(v) => v.is_slice_of(ty),
        };
    }

    pub fn to_sized(&self) -> SizedSliceType {
        return match self {
            Self::Sized(v) => v.clone(),
            _ => panic!("called 'to_sized' on type '{}'", self.id()),
        };
    }

    pub fn to_unsized(&self) -> UnSizedSliceType {
        return match self {
            Self::UnSized(v) => v.clone(),
            _ => panic!("called 'to_unsized' on type '{}'", self.id()),
        };
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.assignable_to(ty),
            Self::UnSized(v) => v.assignable_to(ty),
        };
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.convertable_to(ty),
            Self::UnSized(v) => v.convertable_to(ty),
        };
    }
}

impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Sized(v) => write!(f, "{}", v),
            Self::UnSized(v) => write!(f, "{}", v),
        };
    }
}

impl crate::ToType for SliceType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.clone());
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Slice {
    Sized(SizedSlice),
    UnSized(UnSizedSlice),
}

impl Slice {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Sized(v) => v.to_type(),
            Self::UnSized(v) => v.to_type(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Sized(v) => v.len(),
            Self::UnSized(v) => v.len(),
        };
    }

    pub fn get(&self) -> Vec<crate::Value> {
        return match self {
            Self::Sized(v) => v.get(),
            Self::UnSized(v) => v.get(),
        };
    }

    pub fn set(&mut self, value: crate::Value) {
        return match self {
            Self::Sized(v) => v.set(value),
            Self::UnSized(v) => v.set(value),
        };
    }

    pub fn set_slice(&mut self, value: Vec<crate::Value>) {
        return match self {
            Self::Sized(v) => v.set_slice(value),
            Self::UnSized(v) => v.set_slice(value),
        };
    }

    pub fn set_index(&mut self, index: usize, value: crate::Value) {
        return match self {
            Self::Sized(v) => v.set_index(index, value),
            Self::UnSized(v) => v.set_index(index, value),
        };
    }

    pub fn is_sized(&self) -> bool {
        return match self {
            Self::Sized(_) => true,
            _ => false,
        };
    }

    pub fn is_unsized(&self) -> bool {
        return match self {
            Self::UnSized(_) => true,
            _ => false,
        };
    }

    pub fn to_sized(&self) -> SizedSlice {
        return match self {
            Self::Sized(v) => v.clone(),
            _ => panic!("called 'to_sized' on type '{}'", self.to_type().id()),
        };
    }

    pub fn to_unsized(&self) -> UnSizedSlice {
        return match self {
            Self::UnSized(v) => v.clone(),
            _ => panic!("called 'to_unsized' on type '{}'", self.to_type().id()),
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return match self {
            Self::Sized(v) => v.iter(),
            Self::UnSized(v) => v.iter(),
        };
    }
}

impl crate::Value {
    pub fn set_slice(&mut self, value: Vec<crate::Value>) {
        return match self {
            Self::Slice(v) => v.set_slice(value),
            _ => panic!("called 'set_slice' on '{}'", self.to_type()),
        };
    }
}

impl crate::ToType for Slice {
    fn to_type(&self) -> crate::Type {
        return match self {
            Self::Sized(v) => v.to_type(),
            Self::UnSized(v) => v.to_type(),
        };
    }
}

impl crate::ToValue for Slice {
    fn to_value(self) -> crate::Value {
        return match self {
            Self::Sized(v) => v.to_value(),
            Self::UnSized(v) => v.to_value(),
        };
    }
}

impl std::fmt::Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Sized(v) => write!(f, "{}", v),
            Self::UnSized(v) => write!(f, "{}", v),
        };
    }
}

impl AsRef<[crate::Value]> for Slice {
    fn as_ref(&self) -> &[crate::Value] {
        return match self {
            Self::Sized(v) => v.as_ref(),
            Self::UnSized(v) => v.as_ref(),
        };
    }
}

impl AsMut<[crate::Value]> for Slice {
    fn as_mut(&mut self) -> &mut [crate::Value] {
        return match self {
            Self::Sized(v) => v.as_mut(),
            Self::UnSized(v) => v.as_mut(),
        };
    }
}

impl Deref for Slice {
    type Target = [crate::Value];

    fn deref(&self) -> &Self::Target {
        return match self {
            Self::Sized(v) => v.deref(),
            Self::UnSized(v) => v.deref(),
        };
    }
}

impl DerefMut for Slice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return match self {
            Self::Sized(v) => v.deref_mut(),
            Self::UnSized(v) => v.deref_mut(),
        };
    }
}

impl Index<usize> for Slice {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        return match self {
            Self::Sized(v) => v.index(index),
            Self::UnSized(v) => v.index(index),
        };
    }
}

impl IndexMut<usize> for Slice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match self {
            Self::Sized(v) => v.index_mut(index),
            Self::UnSized(v) => v.index_mut(index),
        };
    }
}
