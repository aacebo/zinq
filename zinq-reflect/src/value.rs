use std::ops::{Index, IndexMut};

use crate::{
    Bool, Enum, Float, Int, Map, Mut, Number, Ref, Seq, Slice, String, Struct, ToType, ToValue,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Bool(Bool),
    Number(Number),
    String(String),
    Ref(Ref),
    Mut(Mut),
    Slice(Slice),
    Struct(Struct),
    Enum(Enum),
    Map(Map),
    Seq(Seq),
    Null,
}

impl Value {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::String(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Struct(v) => v.to_type(),
            Self::Enum(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            Self::Seq(v) => v.to_type(),
            _ => panic!("called 'to_type' on null"),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::String(v) => v.len(),
            Self::Slice(v) => v.len(),
            Self::Map(v) => v.len(),
            Self::Seq(v) => v.len(),
            Self::Ref(v) => v.len(),
            Self::Mut(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.to_type().id()),
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return match self {
            Self::Slice(v) => v.iter(),
            Self::Seq(v) => v.iter(),
            Self::Ref(v) => v.iter(),
            Self::Mut(v) => v.iter(),
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

    pub fn is_ref(&self) -> bool {
        return match self {
            Self::Ref(_) => true,
            _ => false,
        };
    }

    pub fn is_ref_of(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Ref(v) => v.to_type().is_ref_of(ty),
            _ => false,
        };
    }

    pub fn is_mut(&self) -> bool {
        return match self {
            Self::Mut(_) => true,
            _ => false,
        };
    }

    pub fn is_mut_of(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Mut(v) => v.to_type().is_mut_of(ty),
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

    pub fn is_map(&self) -> bool {
        return match self {
            Self::Map(_) => true,
            _ => false,
        };
    }

    pub fn is_seq(&self) -> bool {
        return match self {
            Self::Seq(_) => true,
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
            Self::Ref(v) => v.get().to_bool(),
            Self::Mut(v) => v.get().to_bool(),
            _ => panic!("called 'to_bool' on '{}'", self.to_type()),
        };
    }

    pub fn as_bool(&self) -> &Bool {
        return match self {
            Self::Bool(v) => v,
            Self::Ref(v) => v.as_ref().as_bool(),
            Self::Mut(v) => v.as_ref().as_bool(),
            _ => panic!("called 'as_bool' on '{}'", self.to_type()),
        };
    }

    pub fn to_ref(&self) -> Ref {
        return match self {
            Self::Ref(v) => v.clone(),
            _ => panic!("called 'to_ref' on '{}'", self.to_type()),
        };
    }

    pub fn to_mut(&self) -> Mut {
        return match self {
            Self::Mut(v) => v.clone(),
            Self::Ref(v) => v.get().to_mut(),
            _ => panic!("called 'to_mut' on '{}'", self.to_type()),
        };
    }

    pub fn as_mut(&self) -> &Mut {
        return match self {
            Self::Mut(v) => v,
            Self::Ref(v) => v.as_ref().as_mut(),
            _ => panic!("called 'as_mut' on '{}'", self.to_type()),
        };
    }

    pub fn to_slice(&self) -> Slice {
        return match self {
            Self::Slice(v) => v.clone(),
            Self::Ref(v) => v.get().to_slice(),
            Self::Mut(v) => v.get().to_slice(),
            _ => panic!("called 'to_slice' on '{}'", self.to_type()),
        };
    }

    pub fn as_slice(&self) -> &Slice {
        return match self {
            Self::Slice(v) => v,
            Self::Ref(v) => v.as_ref().as_slice(),
            Self::Mut(v) => v.as_ref().as_slice(),
            _ => panic!("called 'as_slice' on '{}'", self.to_type()),
        };
    }

    pub fn to_struct(&self) -> Struct {
        return match self {
            Self::Struct(v) => v.clone(),
            Self::Ref(v) => v.get().to_struct(),
            Self::Mut(v) => v.get().to_struct(),
            _ => panic!("called 'to_struct' on '{}'", self.to_type()),
        };
    }

    pub fn as_struct(&self) -> &Struct {
        return match self {
            Self::Struct(v) => v,
            Self::Ref(v) => v.as_ref().as_struct(),
            Self::Mut(v) => v.as_ref().as_struct(),
            _ => panic!("called 'as_struct' on '{}'", self.to_type()),
        };
    }

    pub fn to_number(&self) -> Number {
        return match self {
            Self::Number(v) => v.clone(),
            Self::Ref(v) => v.get().to_number(),
            Self::Mut(v) => v.get().to_number(),
            _ => panic!("called 'to_number' on '{}'", self.to_type()),
        };
    }

    pub fn as_number(&self) -> &Number {
        return match self {
            Self::Number(v) => v,
            Self::Ref(v) => v.as_ref().as_number(),
            Self::Mut(v) => v.as_ref().as_number(),
            _ => panic!("called 'as_number' on '{}'", self.to_type()),
        };
    }

    pub fn to_int(&self) -> Int {
        return match self {
            Self::Number(v) => v.to_int(),
            Self::Ref(v) => v.get().to_int(),
            Self::Mut(v) => v.get().to_int(),
            _ => panic!("called 'to_int' on '{}'", self.to_type()),
        };
    }

    pub fn as_int(&self) -> &Int {
        return match self {
            Self::Number(v) => v.as_int(),
            Self::Ref(v) => v.as_ref().as_int(),
            Self::Mut(v) => v.as_ref().as_int(),
            _ => panic!("called 'as_int' on '{}'", self.to_type()),
        };
    }

    pub fn to_float(&self) -> Float {
        return match self {
            Self::Number(v) => v.to_float(),
            Self::Ref(v) => v.get().to_float(),
            Self::Mut(v) => v.get().to_float(),
            _ => panic!("called 'to_float' on '{}'", self.to_type()),
        };
    }

    pub fn as_float(&self) -> &Float {
        return match self {
            Self::Number(v) => v.as_float(),
            Self::Ref(v) => v.as_ref().as_float(),
            Self::Mut(v) => v.as_ref().as_float(),
            _ => panic!("called 'as_float' on '{}'", self.to_type()),
        };
    }

    pub fn to_string(&self) -> String {
        return match self {
            Self::String(v) => v.clone(),
            Self::Ref(v) => v.as_ref().to_string(),
            Self::Mut(v) => v.as_ref().to_string(),
            _ => panic!("called 'to_string' on '{}'", self.to_type()),
        };
    }

    pub fn as_string(&self) -> &String {
        return match self {
            Self::String(v) => v,
            Self::Ref(v) => v.as_ref().as_string(),
            Self::Mut(v) => v.as_ref().as_string(),
            _ => panic!("called 'as_string' on '{}'", self.to_type()),
        };
    }

    pub fn to_enum(&self) -> Enum {
        return match self {
            Self::Enum(v) => v.clone(),
            Self::Ref(v) => v.get().to_enum(),
            Self::Mut(v) => v.get().to_enum(),
            _ => panic!("called 'to_enum' on '{}'", self.to_type()),
        };
    }

    pub fn as_enum(&self) -> &Enum {
        return match self {
            Self::Enum(v) => v,
            Self::Ref(v) => v.as_ref().as_enum(),
            Self::Mut(v) => v.as_ref().as_enum(),
            _ => panic!("called 'as_enum' on '{}'", self.to_type()),
        };
    }

    pub fn to_map(&self) -> Map {
        return match self {
            Self::Map(v) => v.clone(),
            Self::Ref(v) => v.get().to_map(),
            Self::Mut(v) => v.get().to_map(),
            _ => panic!("called 'to_map' on '{}'", self.to_type()),
        };
    }

    pub fn as_map(&self) -> &Map {
        return match self {
            Self::Map(v) => v,
            Self::Ref(v) => v.as_ref().as_map(),
            Self::Mut(v) => v.as_ref().as_map(),
            _ => panic!("called 'as_map' on '{}'", self.to_type()),
        };
    }

    pub fn to_seq(&self) -> Seq {
        return match self {
            Self::Seq(v) => v.clone(),
            Self::Ref(v) => v.get().to_seq(),
            Self::Mut(v) => v.get().to_seq(),
            _ => panic!("called 'to_seq' on '{}'", self.to_type()),
        };
    }

    pub fn as_seq(&self) -> &Seq {
        return match self {
            Self::Seq(v) => v,
            Self::Ref(v) => v.as_ref().as_seq(),
            Self::Mut(v) => v.as_ref().as_seq(),
            _ => panic!("called 'as_seq' on '{}'", self.to_type()),
        };
    }

    pub fn set_key_value(&mut self, key: crate::Value, value: crate::Value) {
        return match self {
            Self::Map(v) => v.set_key_value(key, value),
            Self::Struct(v) => v.set_key_value(&key.to_string(), value),
            Self::Ref(v) => v.set_key_value(key, value),
            Self::Mut(v) => v.set_key_value(key, value),
            _ => panic!("called 'set_key_value' on '{}'", self.to_type()),
        };
    }

    pub fn set_index(&mut self, index: usize, value: crate::Value) {
        return match self {
            Self::Slice(v) => v.set_index(index, value),
            Self::Seq(v) => v.set_index(index, value),
            Self::Ref(v) => v.set_index(index, value),
            Self::Mut(v) => v.set_index(index, value),
            _ => panic!("called 'set_index' on '{}'", self.to_type()),
        };
    }

    pub fn set(&mut self, value: crate::Value) {
        return match self {
            Self::Bool(v) => v.set(value),
            Self::Number(v) => v.set(value),
            Self::String(v) => v.set(value),
            Self::Slice(v) => v.set(value),
            Self::Ref(v) => v.set(value),
            Self::Mut(v) => v.set(value),
            Self::Map(v) => v.set(value),
            Self::Seq(v) => v.set(value),
            _ => panic!("called 'set' on '{}'", self.to_type()),
        };
    }
}

impl crate::TypeOf for Value {
    fn type_of() -> crate::Type {
        return crate::Type::Any;
    }
}

impl crate::ToType for Value {
    fn to_type(&self) -> crate::Type {
        return match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::String(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Struct(v) => v.to_type(),
            Self::Enum(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            _ => panic!("called 'to_type' on null"),
        };
    }
}

impl crate::ToValue for Value {
    fn to_value(self) -> crate::Value {
        return self;
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Mut(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
            Self::Map(v) => write!(f, "{}", v),
            Self::Seq(v) => write!(f, "{}", v),
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
            Self::Seq(v) => v.index(index),
            Self::Ref(v) => v.index(index),
            Self::Mut(v) => v.index(index),
            _ => panic!("called 'Index<usize>::index' on '{}'", self.to_type()),
        };
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match self {
            Self::Slice(v) => v.index_mut(index),
            Self::Seq(v) => v.index_mut(index),
            Self::Ref(v) => v.index_mut(index),
            Self::Mut(v) => v.index_mut(index),
            _ => panic!("called 'IndexMut<usize>::index' on '{}'", self.to_type()),
        };
    }
}

impl Index<&str> for Value {
    type Output = Self;

    fn index(&self, index: &str) -> &Self::Output {
        return match self {
            Self::Struct(v) => v.index(index),
            Self::Map(v) => v.index(&index.to_value()),
            Self::Ref(v) => v.index(&index.to_value()),
            Self::Mut(v) => v.index(&index.to_value()),
            _ => panic!("called 'Index<&str>::index' on '{}'", self.to_type()),
        };
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return match self {
            Self::Struct(v) => v.index_mut(index),
            Self::Map(v) => v.index_mut(&index.to_value()),
            Self::Ref(v) => v.index_mut(&index.to_value()),
            Self::Mut(v) => v.index_mut(&index.to_value()),
            _ => panic!("called 'IndexMut<&str>::index' on '{}'", self.to_type()),
        };
    }
}

impl Index<&Self> for Value {
    type Output = Self;

    fn index(&self, index: &Self) -> &Self::Output {
        return match self {
            Self::Struct(v) => v.index(index.to_string().as_str()),
            Self::Map(v) => v.index(index),
            Self::Slice(v) => v.index(index.to_i32().get() as usize),
            Self::Seq(v) => v.index(index),
            Self::Ref(v) => v.index(index),
            Self::Mut(v) => v.index(index),
            _ => panic!("called 'Index<&Value>::index' on '{}'", self.to_type()),
        };
    }
}

impl IndexMut<&Self> for Value {
    fn index_mut(&mut self, index: &Self) -> &mut Self::Output {
        return match self {
            Self::Struct(v) => v.index_mut(index.to_string().as_str()),
            Self::Map(v) => v.index_mut(index),
            Self::Slice(v) => v.index_mut(index.to_i32().get() as usize),
            Self::Seq(v) => v.index_mut(index),
            Self::Ref(v) => v.index_mut(index),
            Self::Mut(v) => v.index_mut(index),
            _ => panic!(
                "called 'IndexMut<&Value>::index_mut' on '{}'",
                self.to_type()
            ),
        };
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return match self {
            Self::Bool(v) => v.partial_cmp(other.as_bool()),
            Self::Number(v) => v.partial_cmp(other.as_number()),
            Self::String(v) => v.partial_cmp(other.as_string()),
            Self::Ref(v) => v.partial_cmp(&other.to_ref()),
            Self::Mut(v) => v.partial_cmp(other.as_mut()),
            _ => panic!("called 'PartialOrd::partial_cmp' on '{}'", self.to_type()),
        };
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return match self {
            Self::Bool(v) => v.cmp(other.as_bool()),
            Self::Number(v) => v.cmp(other.as_number()),
            Self::String(v) => v.cmp(other.as_string()),
            Self::Ref(v) => v.cmp(&other.to_ref()),
            Self::Mut(v) => v.cmp(other.as_mut()),
            _ => panic!("called 'Ord::cmp' on '{}'", self.to_type()),
        };
    }
}
