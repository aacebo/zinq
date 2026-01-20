use std::sync::atomic::{AtomicUsize, Ordering};

use zinq_reflect::ty::Type;

#[derive(Debug)]
pub struct TypeCell {
    pub ty: Type,
    pub ref_c: AtomicUsize,
}

impl TypeCell {
    pub fn as_type(&self) -> &Type {
        &self.ty
    }

    pub fn to_type(&self) -> Type {
        self.ty.clone()
    }

    pub fn inc_refs(&mut self) {
        self.ref_c.fetch_add(1, Ordering::SeqCst);
    }

    pub fn dec_refs(&mut self) {
        self.ref_c.fetch_min(1, Ordering::SeqCst);
    }

    pub fn ref_count(&self) -> usize {
        self.ref_c.load(Ordering::SeqCst)
    }
}

impl From<Type> for TypeCell {
    fn from(ty: Type) -> Self {
        Self {
            ty,
            ref_c: AtomicUsize::new(0),
        }
    }
}
