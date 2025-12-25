mod builder;
pub mod code;
mod error;

use std::rc::Rc;

pub use builder::*;
pub use code::*;
pub use error::*;

pub type Result<T> = std::result::Result<T, ZinqError>;

impl<T: std::error::Error + 'static> StdError for T {}

pub trait StdError: std::error::Error + 'static {}

#[derive(Debug, Clone)]
pub enum ZinqError {
    Core(Error),
    Std(Rc<dyn StdError>),
}

impl From<Error> for ZinqError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

impl<'a> std::fmt::Display for ZinqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Core(v) => write!(f, "{}", v),
            Self::Std(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for ZinqError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Core(v) => Some(v),
            Self::Std(v) => Some(v.as_ref()),
        }
    }
}
