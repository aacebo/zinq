mod error;
mod location;
mod node;
mod span;
mod token;

pub use error::*;
pub use location::*;
pub use node::*;
pub use span::*;
pub use token::*;

type Result<T> = std::result::Result<T, Error>;
