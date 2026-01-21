use std::sync::atomic::{AtomicU8, Ordering};

use zinq_types::TypeId;

use crate::{GcMark, ObjectId, arena::ArenaId};

///
/// ### GcHeader
/// per-allocation header stored inline before the payload
///
#[repr(C)]
#[derive(Debug)]
pub struct GcHeader {
    ty: TypeId,       // the id of the type of the object instance
    object: ObjectId, // the id of the object instance
    arena: ArenaId,   // the id of the arena that owns the object
    size: u32,        // the total allocation size (header + payload + footer) in bytes
    mark: AtomicU8,   // the color (tri-color mark and sweep)
}

impl GcHeader {
    pub fn ty(&self) -> &TypeId {
        &self.ty
    }

    pub fn object(&self) -> &ObjectId {
        &self.object
    }

    pub fn arena(&self) -> &ArenaId {
        &self.arena
    }

    pub fn size(&self) -> &u32 {
        &self.size
    }

    pub fn mark(&self) -> GcMark {
        let inner = self.mark.load(Ordering::SeqCst);
        GcMark::try_from(inner).unwrap()
    }
}
