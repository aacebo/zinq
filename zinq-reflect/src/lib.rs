pub mod bool;
pub mod number;
pub mod ptr;
pub mod slice;
pub mod string;
mod type_of;
mod value;
mod value_of;

pub use bool::*;
pub use number::*;
pub use ptr::*;
pub use slice::*;
pub use string::*;
pub use type_of::*;
pub use value::*;
pub use value_of::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Bool(BoolType),
    Number(NumberType),
    String(StringType),
    Ptr(PtrType),
    Slice(SliceType),
}

impl Type {
    pub fn id(&self) -> std::any::TypeId {
        return match self {
            Self::Bool(v) => v.id(),
            Self::Number(v) => v.id(),
            Self::String(v) => v.id(),
            Self::Ptr(v) => v.id(),
            Self::Slice(v) => v.id(),
        };
    }

    pub fn name(&self) -> std::string::String {
        return match self {
            Self::Bool(v) => v.name(),
            Self::Number(v) => v.name(),
            Self::String(v) => v.name(),
            Self::Ptr(v) => v.name(),
            Self::Slice(v) => v.name(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Slice(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.name()),
        };
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr(&self) -> bool {
        return match self {
            Self::Ptr(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr_of(&self, _type: Self) -> bool {
        return match self {
            Self::Ptr(v) => v.is_ptr_of(_type),
            _ => false,
        };
    }

    pub fn is_slice(&self) -> bool {
        return match self {
            Self::Slice(_) => true,
            _ => false,
        };
    }

    pub fn is_slice_of(&self, _type: Self) -> bool {
        return match self {
            Self::Slice(v) => v.is_slice_of(_type),
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

    pub fn is_signed(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_signed(),
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match self {
            Self::String(_) => true,
            _ => false,
        };
    }

    pub fn to_bool(&self) -> BoolType {
        return match self {
            Self::Bool(v) => v.clone(),
            _ => panic!("called 'to_bool' on '{}'", self.name()),
        };
    }

    pub fn to_ptr(&self) -> PtrType {
        return match self {
            Self::Ptr(v) => v.clone(),
            _ => panic!("called 'to_ptr' on '{}'", self.name()),
        };
    }

    pub fn to_slice(&self) -> SliceType {
        return match self {
            Self::Slice(v) => v.clone(),
            _ => panic!("called 'to_slice' on '{}'", self.name()),
        };
    }

    pub fn to_number(&self) -> NumberType {
        return match self {
            Self::Number(v) => v.clone(),
            _ => panic!("called 'to_number' on '{}'", self.name()),
        };
    }

    pub fn to_int(&self) -> IntType {
        return match self {
            Self::Number(v) => v.to_int(),
            _ => panic!("called 'to_int' on '{}'", self.name()),
        };
    }

    pub fn to_float(&self) -> FloatType {
        return match self {
            Self::Number(v) => v.to_float(),
            _ => panic!("called 'to_float' on '{}'", self.name()),
        };
    }

    pub fn to_string(&self) -> StringType {
        return match self {
            Self::String(v) => v.clone(),
            _ => panic!("called 'to_string' on '{}'", self.name()),
        };
    }

    pub fn assignable_to(&self, _type: Self) -> bool {
        return match self {
            Self::Bool(v) => v.assignable_to(_type),
            Self::Number(v) => v.assignable_to(_type),
            Self::String(v) => v.assignable_to(_type),
            Self::Ptr(v) => v.assignable_to(_type),
            Self::Slice(v) => v.assignable_to(_type),
        };
    }

    pub fn convertable_to(&self, _type: Self) -> bool {
        return match self {
            Self::Bool(v) => v.convertable_to(_type),
            Self::Number(v) => v.convertable_to(_type),
            Self::String(v) => v.convertable_to(_type),
            Self::Ptr(v) => v.convertable_to(_type),
            Self::Slice(v) => v.convertable_to(_type),
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Ptr(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
        };
    }
}
