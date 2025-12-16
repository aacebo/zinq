mod dynamic;
mod list;
mod not_found;
mod text;

pub use dynamic::*;
pub use list::*;
pub use not_found::*;
pub use text::*;

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
    Dyn(DynError),
    NotFound(NotFoundError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(err) => write!(f, "{}", err),
            Self::List(err) => write!(f, "{}", err),
            Self::Dyn(err) => write!(f, "{}", err),
            Self::NotFound(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Text(err) => err.source(),
            Self::List(err) => err.source(),
            Self::Dyn(err) => err.source(),
            Self::NotFound(err) => err.source(),
        }
    }
}
