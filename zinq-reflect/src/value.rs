use std::ops::{Index, IndexMut};

use crate::{Bool, Enum, Float, Int, Number, Ref, Slice, String, Struct, Type, TypeOf};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Bool(Bool),
    Number(Number),
    String(String),
    Ref(Ref),
    Slice(Slice),
    Struct(Struct),
    Enum(Enum),
    Null,
}

impl Value {
    pub fn to_type(&self) -> Type {
        return match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::String(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Struct(v) => v.to_type(),
            Self::Enum(v) => v.to_type(),
            _ => panic!("called 'to_type' on null"),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::String(v) => v.len(),
            Self::Slice(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.to_type().id()),
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return match self {
            Self::Slice(v) => v.iter(),
            _ => panic!("called 'iter' on '{}'", self.to_type()),
        };
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_true(&self) -> bool {
        return match self {
            Self::Bool(v) => v.is_true(),
            _ => false,
        };
    }

    pub fn is_false(&self) -> bool {
        return match self {
            Self::Bool(v) => v.is_false(),
            _ => false,
        };
    }

    pub fn is_ptr(&self) -> bool {
        return match self {
            Self::Ref(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr_of(&self, ty: Type) -> bool {
        return match self {
            Self::Ref(v) => v.to_type().is_ptr_of(ty),
            _ => false,
        };
    }

    pub fn is_slice(&self) -> bool {
        return match self {
            Self::Slice(_) => true,
            _ => false,
        };
    }

    pub fn is_struct(&self) -> bool {
        return match self {
            Self::Struct(_) => true,
            _ => false,
        };
    }

    pub fn is_number(&self) -> bool {
        return match self {
            Self::Number(_) => true,
            _ => false,
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_int(),
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_float(),
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match self {
            Self::String(_) => true,
            _ => false,
        };
    }

    pub fn is_enum(&self) -> bool {
        return match self {
            Self::Enum(_) => true,
            _ => false,
        };
    }

    pub fn is_null(&self) -> bool {
        return match self {
            Self::Null => true,
            _ => false,
        };
    }

    pub fn to_bool(&self) -> Bool {
        return match self {
            Self::Bool(v) => v.clone(),
            _ => panic!("called 'to_bool' on '{}'", self.to_type()),
        };
    }

    pub fn to_ptr(&self) -> Ref {
        return match self {
            Self::Ref(v) => v.clone(),
            _ => panic!("called 'to_ptr' on '{}'", self.to_type()),
        };
    }

    pub fn to_slice(&self) -> Slice {
        return match self {
            Self::Slice(v) => v.clone(),
            _ => panic!("called 'to_slice' on '{}'", self.to_type()),
        };
    }

    pub fn to_struct(&self) -> Struct {
        return match self {
            Self::Struct(v) => v.clone(),
            _ => panic!("called 'to_struct' on '{}'", self.to_type()),
        };
    }

    pub fn to_number(&self) -> Number {
        return match self {
            Self::Number(v) => v.clone(),
            _ => panic!("called 'to_number' on '{}'", self.to_type()),
        };
    }

    pub fn to_int(&self) -> Int {
        return match self {
            Self::Number(v) => v.to_int(),
            _ => panic!("called 'to_int' on '{}'", self.to_type()),
        };
    }

    pub fn to_float(&self) -> Float {
        return match self {
            Self::Number(v) => v.to_float(),
            _ => panic!("called 'to_float' on '{}'", self.to_type()),
        };
    }

    pub fn to_string(&self) -> String {
        return match self {
            Self::String(v) => v.clone(),
            _ => panic!("called 'to_string' on '{}'", self.to_type()),
        };
    }

    pub fn to_enum(&self) -> Enum {
        return match self {
            Self::Enum(v) => v.clone(),
            _ => panic!("called 'to_enum' on '{}'", self.to_type()),
        };
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
            Self::Null => write!(f, "<null>"),
        };
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        return self;
    }
}

impl AsMut<Value> for Value {
    fn as_mut(&mut self) -> &mut Value {
        return self;
    }
}

impl Index<usize> for Value {
    type Output = Self;

    fn index(&self, index: usize) -> &Self::Output {
        return match self {
            Self::Slice(v) => v.index(index),
            _ => panic!("called 'index' on '{}'", self.to_type()),
        };
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match self {
            Self::Slice(v) => v.index_mut(index),
            _ => panic!("called 'index' on '{}'", self.to_type()),
        };
    }
}

impl Index<&str> for Value {
    type Output = Self;

    fn index(&self, index: &str) -> &Self::Output {
        return match self {
            Self::Struct(v) => v.index(index),
            _ => panic!("called 'index' on '{}'", self.to_type()),
        };
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return match self {
            Self::Struct(v) => v.index_mut(index),
            _ => panic!("called 'index' on '{}'", self.to_type()),
        };
    }
}
