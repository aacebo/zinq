mod bounds;
pub mod expr;
pub mod fields;
mod generics;
pub mod param;
pub mod pat;
mod path;
pub mod spread;
pub mod stmt;
pub mod ty;
mod use_path;
mod variant;
mod visibility;

pub use bounds::*;
pub use generics::*;
pub use path::*;
pub use use_path::*;
pub use variant::*;
pub use visibility::*;

use zinq_error::Result;

pub trait Node {
    fn name(&self) -> &str;
    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized;
}

pub trait Visitor<N: Node> {
    fn visit(&mut self, node: &N) -> Result<()>;
}
