mod builder;
mod code;
mod error;

pub use builder::*;
pub use code::*;
pub use error::*;

pub type Result<T> = std::result::Result<T, Error>;
pub trait StdError: std::error::Error + 'static {}

pub trait ToError {
    fn to_error(self) -> Error;
}

impl<T: std::error::Error + 'static> StdError for T {}

impl<T: StdError> ToError for T {
    fn to_error(self) -> Error {
        Error::new().with_source(self).build()
    }
}
