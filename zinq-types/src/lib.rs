pub mod error;
mod field;
mod r#impl;
mod param;
mod path;
mod size;
pub mod ty;
mod type_cell;
mod type_path;
mod type_ptr;
mod type_registry;
mod variant;

pub use field::*;
pub use r#impl::*;
pub use param::*;
pub use path::*;
pub use size::*;
pub use type_cell::*;
pub use type_path::*;
pub use type_ptr::*;
pub use type_registry::*;
pub use variant::*;

pub trait Resolve {
    type Output;

    fn resolve(self, registry: &TypeRegistry) -> zinq_error::Result<&Self::Output>;
}

impl Resolve for TypePtr {
    type Output = ty::Type;

    fn resolve(self, registry: &TypeRegistry) -> zinq_error::Result<&'_ Self::Output> {
        registry.get_or_err(&self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(zinq_hash::Object);

impl std::fmt::Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub trait ZinqType {
    fn name(&self) -> String;
    fn size(&self) -> Size;
    fn refs(&self) -> Box<[TypePtr]> {
        Box::new([])
    }

    fn ptr(&self) -> TypePtr
    where
        Self: Sized,
    {
        TypePtr::from(self)
    }

    fn id(&self) -> TypeId {
        let mut hasher = zinq_hash::Hasher::new();

        if let Some(module) = &self.module() {
            hasher.push(&module);
        }

        hasher.push_str(&self.name());
        TypeId(hasher.build())
    }

    fn module(&self) -> Option<Path> {
        None
    }
}
