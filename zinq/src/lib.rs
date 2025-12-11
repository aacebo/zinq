mod node;
mod location;
mod span;
mod token;
mod error;

pub use node::*;
pub use location::*;
pub use span::*;
pub use token::*;
pub use error::*;

type Result<T> = std::result::Result<T, Error>;