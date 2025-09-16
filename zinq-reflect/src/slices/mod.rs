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
    pub fn id(&self) -> std::any::TypeId {
        return match self {
            Self::Sized(v) => v.id(),
            Self::UnSized(v) => v.id(),
        };
    }

    pub fn name(&self) -> &str {
        return match self {
            Self::Sized(v) => v.name(),
            Self::UnSized(v) => v.name(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Sized(v) => v.len(),
            Self::UnSized(_) => panic!("called 'len' on type '{}'", self.name()),
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Slice(self.clone());
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

    pub fn is_slice_of(&self, _type: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.is_slice_of(_type),
            Self::UnSized(v) => v.is_slice_of(_type),
        };
    }

    pub fn to_sized(&self) -> SizedSliceType {
        return match self {
            Self::Sized(v) => v.clone(),
            _ => panic!("called 'to_sized' on type '{}'", self.name()),
        };
    }

    pub fn to_unsized(&self) -> UnSizedSliceType {
        return match self {
            Self::UnSized(v) => v.clone(),
            _ => panic!("called 'to_unsized' on type '{}'", self.name()),
        };
    }

    pub fn assignable_to(&self, _type: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.assignable_to(_type),
            Self::UnSized(v) => v.assignable_to(_type),
        };
    }

    pub fn convertable_to(&self, _type: crate::Type) -> bool {
        return match self {
            Self::Sized(v) => v.convertable_to(_type),
            Self::UnSized(v) => v.convertable_to(_type),
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
            _ => panic!("called 'to_sized' on type '{}'", self.to_type().name()),
        };
    }

    pub fn to_unsized(&self) -> UnSizedSlice {
        return match self {
            Self::UnSized(v) => v.clone(),
            _ => panic!("called 'to_unsized' on type '{}'", self.to_type().name()),
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return match self {
            Self::Sized(v) => v.iter(),
            Self::UnSized(v) => v.iter(),
        };
    }
}

impl crate::Reflect for Slice {
    fn reflect(self) -> crate::Value {
        return match self {
            Self::Sized(v) => v.reflect(),
            Self::UnSized(v) => v.reflect(),
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
