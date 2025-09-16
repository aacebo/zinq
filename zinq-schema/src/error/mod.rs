mod basic;
mod field;
mod group;

pub use basic::*;
pub use field::*;
pub use group::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    Basic(BasicError),
    Field(FieldError),
    Group(GroupError),
}

impl Error {
    pub fn len(&self) -> usize {
        return match self {
            Self::Basic(v) => v.len(),
            Self::Field(v) => v.len(),
            Self::Group(v) => v.len(),
        };
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Basic(v) => write!(f, "{}", v),
            Self::Field(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        };
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Self::Basic(v) => v.source(),
            Self::Field(v) => v.source(),
            Self::Group(v) => v.source(),
        };
    }
}
