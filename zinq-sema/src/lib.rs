mod arena;
mod context;
pub mod expr;
mod id;
pub mod stmt;

pub use arena::*;
pub use context::*;
pub use id::*;

use zinq_error::Result;

pub trait Build {
    type Output;

    fn build(&self, ctx: &mut Context) -> Result<Self::Output>;
}
