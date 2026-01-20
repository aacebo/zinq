use std::sync::atomic::{AtomicPtr, Ordering};

use zinq_types::TypeId;

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Value {
    ty: TypeId,
    length: u32,
    align: u32,
    flags: u32,
    reserved: u64,
    ptr: AtomicPtr<u8>,
}

impl Value {
    /// Type describing what this Value means (Int, Bool, Obj, Str, etc.).
    ///
    /// Offset: 16
    /// Size:   4
    pub fn ty(&self) -> &TypeId {
        &self.ty
    }

    /// Payload size (bytes) for heap data, or 0 for immediates.
    ///
    /// Offset: 8
    /// Size:   4
    pub fn len(&self) -> u32 {
        self.length
    }

    /// Alignment (bytes) of the heap payload (power of two), or 0 for immediates.
    ///
    /// Offset: 12
    /// Size:   4
    pub fn align(&self) -> u32 {
        self.align
    }

    /// Extra bits (e.g., GC color, inline small-int tag bits, etc.).
    ///
    /// Offset: 20
    /// Size:   4
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Reserved for future use; keeps total size stable and aligned.
    ///
    /// Offset: 24
    /// Size:   8
    pub fn reserved(&self) -> u64 {
        self.reserved
    }

    /// Atomic pointer to the heap payload (or null for immediates).
    ///
    /// Offset: 0
    /// Size:   8 (on 64-bit)
    /// Align:  8
    pub fn ptr(&self) -> *mut u8 {
        self.ptr.load(Ordering::SeqCst)
    }
}
