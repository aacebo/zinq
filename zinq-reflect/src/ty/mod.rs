mod alias_type;
mod array_type;
mod bool_type;
mod enum_type;
mod float_type;
mod fn_type;
mod int_type;
mod mod_type;
mod ptr_type;
mod string_type;
mod struct_type;
mod tuple_type;
mod uint_type;

pub use alias_type::*;
pub use array_type::*;
pub use bool_type::*;
pub use enum_type::*;
pub use float_type::*;
pub use fn_type::*;
pub use int_type::*;
pub use mod_type::*;
pub use ptr_type::*;
pub use string_type::*;
pub use struct_type::*;
pub use tuple_type::*;
pub use uint_type::*;

use crate::{Path, Size};

pub trait ZinqType {
    fn name(&self) -> String;
    fn size(&self) -> Size;
    fn module(&self) -> Option<Path> {
        None
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Alias(AliasType),
    Array(ArrayType),
    Bool(BoolType),
    String(StringType),
    Int(IntType),
    UInt(UIntType),
    Float(FloatType),
    Ptr(PtrType),
    Tuple(TupleType),
    Struct(StructType),
    Mod(ModType),
    Fn(FnType),
    Enum(EnumType),
}

impl Type {
    pub fn is_alias(&self) -> bool {
        match self {
            Self::Alias(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Self::Enum(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Self::Fn(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_mod(&self) -> bool {
        match self {
            Self::Mod(_) => true,
            _ => false,
        }
    }

    pub fn is_ptr(&self) -> bool {
        match self {
            Self::Ptr(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_uint(&self) -> bool {
        match self {
            Self::UInt(_) => true,
            _ => false,
        }
    }

    pub fn as_alias(&self) -> &AliasType {
        match self {
            Self::Alias(v) => v,
            v => panic!("{}", format!("expected AliasType, received {}", v.name())),
        }
    }

    pub fn as_array(&self) -> &ArrayType {
        match self {
            Self::Array(v) => v,
            v => panic!("{}", format!("expected ArrayType, received {}", v.name())),
        }
    }

    pub fn as_bool(&self) -> &BoolType {
        match self {
            Self::Bool(v) => v,
            v => panic!("{}", format!("expected BoolType, received {}", v.name())),
        }
    }

    pub fn as_enum(&self) -> &EnumType {
        match self {
            Self::Enum(v) => v,
            v => panic!("{}", format!("expected EnumType, received {}", v.name())),
        }
    }

    pub fn as_float(&self) -> &FloatType {
        match self {
            Self::Float(v) => v,
            v => panic!("{}", format!("expected FloatType, received {}", v.name())),
        }
    }

    pub fn as_fn(&self) -> &FnType {
        match self {
            Self::Fn(v) => v,
            v => panic!("{}", format!("expected FnType, received {}", v.name())),
        }
    }

    pub fn as_int(&self) -> &IntType {
        match self {
            Self::Int(v) => v,
            v => panic!("{}", format!("expected IntType, received {}", v.name())),
        }
    }

    pub fn as_mod(&self) -> &ModType {
        match self {
            Self::Mod(v) => v,
            v => panic!("{}", format!("expected ModType, received {}", v.name())),
        }
    }

    pub fn as_ptr(&self) -> &PtrType {
        match self {
            Self::Ptr(v) => v,
            v => panic!("{}", format!("expected PtrType, received {}", v.name())),
        }
    }

    pub fn as_string(&self) -> &StringType {
        match self {
            Self::String(v) => v,
            v => panic!("{}", format!("expected StringType, received {}", v.name())),
        }
    }

    pub fn as_struct(&self) -> &StructType {
        match self {
            Self::Struct(v) => v,
            v => panic!("{}", format!("expected StructType, received {}", v.name())),
        }
    }

    pub fn as_tuple(&self) -> &TupleType {
        match self {
            Self::Tuple(v) => v,
            v => panic!("{}", format!("expected TupleType, received {}", v.name())),
        }
    }

    pub fn as_uint(&self) -> &UIntType {
        match self {
            Self::UInt(v) => v,
            v => panic!("{}", format!("expected UIntType, received {}", v.name())),
        }
    }
}

impl ZinqType for Type {
    fn name(&self) -> String {
        match self {
            Self::Alias(v) => v.name(),
            Self::Array(v) => v.name(),
            Self::Bool(v) => v.name(),
            Self::String(v) => v.name(),
            Self::Int(v) => v.name(),
            Self::UInt(v) => v.name(),
            Self::Float(v) => v.name(),
            Self::Ptr(v) => v.name(),
            Self::Tuple(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Mod(v) => v.name(),
            Self::Fn(v) => v.name(),
            Self::Enum(v) => v.name(),
        }
    }

    fn module(&self) -> Option<Path> {
        match self {
            Self::Alias(v) => v.module(),
            Self::Array(v) => v.module(),
            Self::Bool(v) => v.module(),
            Self::String(v) => v.module(),
            Self::Int(v) => v.module(),
            Self::UInt(v) => v.module(),
            Self::Float(v) => v.module(),
            Self::Ptr(v) => v.module(),
            Self::Tuple(v) => v.module(),
            Self::Struct(v) => v.module(),
            Self::Mod(v) => v.module(),
            Self::Fn(v) => v.module(),
            Self::Enum(v) => v.module(),
        }
    }

    fn size(&self) -> Size {
        match self {
            Self::Alias(v) => v.size(),
            Self::Array(v) => v.size(),
            Self::Bool(v) => v.size(),
            Self::String(v) => v.size(),
            Self::Int(v) => v.size(),
            Self::UInt(v) => v.size(),
            Self::Float(v) => v.size(),
            Self::Ptr(v) => v.size(),
            Self::Tuple(v) => v.size(),
            Self::Struct(v) => v.size(),
            Self::Mod(v) => v.size(),
            Self::Fn(v) => v.size(),
            Self::Enum(v) => v.size(),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alias(v) => write!(f, "{}", v),
            Self::Array(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::UInt(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::Ptr(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Mod(v) => write!(f, "{}", v),
            Self::Fn(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
        }
    }
}
