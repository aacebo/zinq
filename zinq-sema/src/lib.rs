mod arena;
mod context;
pub mod expr;
pub mod id;
pub mod stmt;

pub use arena::*;
pub use context::*;

use zinq_error::Result;

pub trait Build {
    type Output;

    fn build(&self, ctx: &mut Context) -> Result<Self::Output>;
}
