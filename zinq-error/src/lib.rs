mod builder;
pub mod category;
pub mod code;
mod error;

pub use builder::*;
pub use category::ErrorCategory;
pub use code::ErrorCode;
pub use error::*;

pub type Result<T> = std::result::Result<T, Error>;
pub trait StdError: std::error::Error + 'static {}

pub trait ToError {
    fn to_error(self) -> Error;
}

impl<T: std::error::Error + 'static> StdError for T {}

impl<T: StdError> ToError for T {
    fn to_error(self) -> Error {
        Error::new().source(self).build()
    }
}

pub trait ZinqError: std::error::Error {
    fn code(&self) -> &ErrorCode;
    fn category(&self) -> &ErrorCategory;
    fn message(&self) -> Option<&str>;
    fn source(&self) -> Option<&dyn StdError>;
    fn children(&self) -> &[Self]
    where
        Self: Sized;
}
