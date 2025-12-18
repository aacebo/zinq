mod group;
mod ident;
mod keyword;
mod literal;
mod punct;

pub use group::*;
pub use ident::*;
pub use keyword::*;
pub use literal::*;
pub use punct::*;

#[derive(Debug, Clone)]
pub enum Any {
    Keyword(Keyword),
}
