mod error;
mod error_group;

pub use error::*;
pub use error_group::*;

#[cfg(feature = "macros")]
pub use zinq_error_macros::*;

pub trait ToError {
    fn to_error(&self) -> ZinqError;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ZinqError {
    Error(Error),
    Group(ErrorGroup),
}

impl ZinqError {
    pub fn len(&self) -> usize {
        return match self {
            Self::Error(v) => v.len(),
            Self::Group(v) => v.len(),
        };
    }
}

impl std::fmt::Display for ZinqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Error(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        };
    }
}

impl std::error::Error for ZinqError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Self::Error(v) => v.source(),
            Self::Group(v) => v.source(),
        };
    }
}
