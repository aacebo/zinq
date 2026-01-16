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

impl From<Error> for ZinqError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

impl From<std::fmt::Error> for ZinqError {
    fn from(value: std::fmt::Error) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::string::FromUtf8Error> for ZinqError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::string::ParseError> for ZinqError {
    fn from(value: std::string::ParseError) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::str::Utf8Error> for ZinqError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::str::ParseBoolError> for ZinqError {
    fn from(value: std::str::ParseBoolError) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::num::ParseIntError> for ZinqError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Std(Rc::new(value))
    }
}

impl From<std::num::ParseFloatError> for ZinqError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::Std(Rc::new(value))
    }
}
