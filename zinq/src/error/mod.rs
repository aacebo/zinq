mod any;
mod list;
mod not_found;
mod text;

pub use any::*;
pub use list::*;
pub use not_found::*;
pub use text::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ToError {
    fn to_error(self) -> Error;
}

impl<T: std::error::Error + 'static> ToError for T {
    fn to_error(self) -> Error {
        AnyError::new(self).into()
    }
}

pub trait AsError {
    fn as_error(&self) -> &dyn std::error::Error;
}

impl<T: std::error::Error> AsError for T {
    fn as_error(&self) -> &dyn std::error::Error {
        self
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Text(TextError),
    List(ListError),
    NotFound(NotFoundError),
    Other(AnyError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(err) => write!(f, "{}", err),
            Self::List(err) => write!(f, "{}", err),
            Self::NotFound(err) => write!(f, "{}", err),
            Self::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Text(err) => err.source(),
            Self::List(err) => err.source(),
            Self::NotFound(err) => err.source(),
            Self::Other(err) => err.source(),
        }
    }
}
