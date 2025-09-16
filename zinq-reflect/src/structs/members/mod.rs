mod field;
mod method;
mod param;
mod visibility;

pub use field::*;
pub use method::*;
pub use param::*;
pub use visibility::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MemberType {
    Field(FieldType),
    Method(MethodType),
}

impl MemberType {
    pub fn vis(&self) -> Visibility {
        return match self {
            Self::Field(v) => v.vis(),
            Self::Method(v) => v.vis(),
        };
    }

    pub fn name(&self) -> &str {
        return match self {
            Self::Field(v) => v.name(),
            Self::Method(v) => v.name(),
        };
    }

    pub fn is_field(&self) -> bool {
        return match self {
            Self::Field(_) => true,
            _ => false,
        };
    }

    pub fn is_method(&self) -> bool {
        return match self {
            Self::Method(_) => true,
            _ => false,
        };
    }

    pub fn to_field(&self) -> FieldType {
        return match self {
            Self::Field(v) => v.clone(),
            _ => panic!("called 'to_field' on 'method' member"),
        };
    }

    pub fn to_method(&self) -> MethodType {
        return match self {
            Self::Method(v) => v.clone(),
            _ => panic!("called 'to_method' on 'field' member"),
        };
    }
}

impl std::fmt::Display for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Field(v) => write!(f, "{}", v),
            Self::Method(v) => write!(f, "{}", v),
        };
    }
}
