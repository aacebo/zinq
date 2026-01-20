use std::sync::atomic::AtomicUsize;

use zinq_reflect::ty::Type;

#[derive(Debug)]
pub struct TypeEntry {
    pub ty: Type,
    pub ref_count: AtomicUsize,
}

impl From<Type> for TypeEntry {
    fn from(ty: Type) -> Self {
        Self {
            ty,
            ref_count: AtomicUsize::new(0),
        }
    }
}
