mod _struct;
mod bool;
pub mod build;
pub mod enums;
pub mod fields;
mod layout;
mod method;
mod module;
pub mod numbers;
mod param;
mod ptr;
pub mod slices;
mod string;
mod tuple;
mod type_id;
mod type_of;
mod value;
mod value_of;
mod visibility;

pub use _struct::*;
pub use bool::*;
pub use enums::*;
pub use fields::*;
pub use layout::*;
pub use method::*;
pub use module::*;
pub use numbers::*;
pub use param::*;
pub use ptr::*;
pub use slices::*;
pub use string::*;
pub use tuple::*;
pub use type_id::*;
pub use type_of::*;
pub use value::*;
pub use value_of::*;
pub use visibility::*;

#[cfg(feature = "macros")]
pub use zinq_reflect_macros::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Bool(BoolType),
    Enum(EnumType),
    Number(NumberType),
    String(StringType),
    Ptr(PtrType),
    Slice(SliceType),
    Struct(StructType),
    Tuple(TupleType),
}

impl Type {
    pub fn id(&self) -> TypeId {
        return match self {
            Self::Bool(v) => v.id(),
            Self::Enum(v) => v.id(),
            Self::Number(v) => v.id(),
            Self::String(v) => v.id(),
            Self::Ptr(v) => v.id(),
            Self::Slice(v) => v.id(),
            Self::Struct(v) => v.id(),
            Self::Tuple(v) => v.id(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Enum(v) => v.len(),
            Self::Slice(v) => v.len(),
            Self::Struct(v) => v.fields().len(),
            Self::Tuple(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.id()),
        };
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_enum(&self) -> bool {
        return match self {
            Self::Enum(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr(&self) -> bool {
        return match self {
            Self::Ptr(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr_of(&self, ty: Self) -> bool {
        return match self {
            Self::Ptr(v) => v.is_ptr_of(ty),
            _ => false,
        };
    }

    pub fn is_slice(&self) -> bool {
        return match self {
            Self::Slice(_) => true,
            _ => false,
        };
    }

    pub fn is_slice_of(&self, ty: Self) -> bool {
        return match self {
            Self::Slice(v) => v.is_slice_of(ty),
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

    pub fn is_tuple(&self) -> bool {
        return match self {
            Self::Tuple(_) => true,
            _ => false,
        };
    }

    pub fn to_bool(&self) -> BoolType {
        return match self {
            Self::Bool(v) => v.clone(),
            _ => panic!("called 'to_bool' on '{}'", self.id()),
        };
    }

    pub fn to_enum(&self) -> EnumType {
        return match self {
            Self::Enum(v) => v.clone(),
            _ => panic!("called 'to_enum' on '{}'", self.id()),
        };
    }

    pub fn to_ptr(&self) -> PtrType {
        return match self {
            Self::Ptr(v) => v.clone(),
            _ => panic!("called 'to_ptr' on '{}'", self.id()),
        };
    }

    pub fn to_slice(&self) -> SliceType {
        return match self {
            Self::Slice(v) => v.clone(),
            _ => panic!("called 'to_slice' on '{}'", self.id()),
        };
    }

    pub fn to_struct(&self) -> StructType {
        return match self {
            Self::Struct(v) => v.clone(),
            _ => panic!("called 'to_struct' on '{}'", self.id()),
        };
    }

    pub fn to_number(&self) -> NumberType {
        return match self {
            Self::Number(v) => v.clone(),
            _ => panic!("called 'to_number' on '{}'", self.id()),
        };
    }

    pub fn to_int(&self) -> IntType {
        return match self {
            Self::Number(v) => v.to_int(),
            _ => panic!("called 'to_int' on '{}'", self.id()),
        };
    }

    pub fn to_float(&self) -> FloatType {
        return match self {
            Self::Number(v) => v.to_float(),
            _ => panic!("called 'to_float' on '{}'", self.id()),
        };
    }

    pub fn to_string(&self) -> StringType {
        return match self {
            Self::String(v) => v.clone(),
            _ => panic!("called 'to_string' on '{}'", self.id()),
        };
    }

    pub fn to_tuple(&self) -> TupleType {
        return match self {
            Self::Tuple(v) => v.clone(),
            _ => panic!("called 'to_tuple' on '{}'", self.id()),
        };
    }

    pub fn assignable_to(&self, ty: Self) -> bool {
        return match self {
            Self::Bool(v) => v.assignable_to(ty),
            Self::Enum(v) => v.assignable_to(ty),
            Self::Number(v) => v.assignable_to(ty),
            Self::String(v) => v.assignable_to(ty),
            Self::Ptr(v) => v.assignable_to(ty),
            Self::Slice(v) => v.assignable_to(ty),
            Self::Struct(v) => v.assignable_to(ty),
            Self::Tuple(v) => v.assignable_to(ty),
        };
    }

    pub fn convertable_to(&self, ty: Self) -> bool {
        return match self {
            Self::Bool(v) => v.convertable_to(ty),
            Self::Enum(v) => v.convertable_to(ty),
            Self::Number(v) => v.convertable_to(ty),
            Self::String(v) => v.convertable_to(ty),
            Self::Ptr(v) => v.convertable_to(ty),
            Self::Slice(v) => v.convertable_to(ty),
            Self::Struct(v) => v.convertable_to(ty),
            Self::Tuple(v) => v.convertable_to(ty),
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Ptr(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
        };
    }
}
