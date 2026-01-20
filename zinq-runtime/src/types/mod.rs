mod cell;
mod registry;

pub use cell::*;
pub use registry::*;

use zinq_error::Result;
use zinq_reflect::{TypePtr, ty::Type};

pub trait Resolve {
    type Output;

    fn resolve(self, registry: &TypeRegistry) -> Result<&Self::Output>;
}

impl Resolve for TypePtr {
    type Output = Type;

    fn resolve(self, registry: &TypeRegistry) -> Result<&'_ Self::Output> {
        registry.get_or_err(&self.id)
    }
}
