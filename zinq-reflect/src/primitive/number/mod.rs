pub mod float;
pub mod int;

pub use float::*;
pub use int::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NumberType {
    Int(IntType),
    Float(FloatType),
}

impl NumberType {
    pub fn id(&self) -> crate::TypeId {
        return match self {
            Self::Int(v) => v.id(),
            Self::Float(v) => v.id(),
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Number(self.clone());
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Int(_) => true,
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Float(_) => true,
            _ => false,
        };
    }

    pub fn is_signed(&self) -> bool {
        return match self {
            Self::Int(v) => v.is_signed(),
            Self::Float(_) => true,
        };
    }

    pub fn to_int(&self) -> IntType {
        return match self {
            Self::Int(v) => v.clone(),
            _ => panic!("called 'to_int' on type '{}'", self.id()),
        };
    }

    pub fn to_float(&self) -> FloatType {
        return match self {
            Self::Float(v) => v.clone(),
            _ => panic!("called 'to_float' on type '{}'", self.id()),
        };
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Int(v) => v.assignable_to(ty),
            Self::Float(v) => v.assignable_to(ty),
        };
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Int(v) => v.convertable_to(ty),
            Self::Float(v) => v.convertable_to(ty),
        };
    }
}

impl std::fmt::Display for NumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Int(Int),
    Float(Float),
}

impl Number {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Int(v) => v.to_type(),
            Self::Float(v) => v.to_type(),
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Int(_) => true,
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Float(_) => true,
            _ => false,
        };
    }

    pub fn to_int(&self) -> Int {
        return match self {
            Self::Int(v) => v.clone(),
            _ => panic!("called 'to_int' on type '{}'", self.to_type().id()),
        };
    }

    pub fn to_float(&self) -> Float {
        return match self {
            Self::Float(v) => v.clone(),
            _ => panic!("called 'to_float' on type '{}'", self.to_type().id()),
        };
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return match self {
            Self::Float(_) => panic!("called 'Ord::cmp' on Float"),
            Self::Int(v) => v.cmp(&other.to_int()),
        };
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return match self {
            Self::Float(_) => None,
            Self::Int(v) => v.partial_cmp(&other.to_int()),
        };
    }
}

impl crate::Reflect for Number {
    fn reflect(self) -> crate::Value {
        return match self {
            Self::Int(v) => v.reflect(),
            Self::Float(v) => v.reflect(),
        };
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
        };
    }
}
