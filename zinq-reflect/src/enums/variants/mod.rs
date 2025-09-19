mod _struct;
mod tuple;
mod unit;

pub use _struct::*;
pub use tuple::*;
pub use unit::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variant {
    Unit(UnitVariant),
    Struct(StructVariant),
    Tuple(TupleVariant),
}

impl Variant {
    pub fn name(&self) -> &str {
        return match self {
            Self::Unit(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Tuple(v) => v.name(),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Struct(v) => v.len(),
            Self::Tuple(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.name()),
        };
    }

    pub fn is_unit(&self) -> bool {
        return match self {
            Self::Unit(_) => true,
            _ => false,
        };
    }

    pub fn is_struct(&self) -> bool {
        return match self {
            Self::Struct(_) => true,
            _ => false,
        };
    }

    pub fn is_tuple(&self) -> bool {
        return match self {
            Self::Tuple(_) => true,
            _ => false,
        };
    }

    pub fn to_unit(&self) -> UnitVariant {
        return match self {
            Self::Unit(v) => v.clone(),
            _ => panic!("called 'to_unit' on '{}'", self.name()),
        };
    }

    pub fn to_struct(&self) -> StructVariant {
        return match self {
            Self::Struct(v) => v.clone(),
            _ => panic!("called 'to_struct' on '{}'", self.name()),
        };
    }

    pub fn to_tuple(&self) -> TupleVariant {
        return match self {
            Self::Tuple(v) => v.clone(),
            _ => panic!("called 'to_tuple' on '{}'", self.name()),
        };
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Unit(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
        };
    }
}
