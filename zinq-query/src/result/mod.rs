mod failure;
mod success;

pub use failure::*;
pub use success::*;

pub type Result<T> = std::result::Result<Success<T>, Failure>;
