use std::{ptr::NonNull, sync::atomic::AtomicU8};

use zinq_types::Layout;

use crate::arena::ArenaId;

///
/// ### GcMeta
/// value metadata
///
#[repr(C)]
#[derive(Debug)]
pub struct GcMeta {
    pub mark: AtomicU8,
    pub arena: ArenaId,
    pub ty: NonNull<GcTypeMeta>,
}

///
/// ### GcTypeMeta
/// type metadata
///
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GcTypeMeta {
    pub layout: Layout,
    pub mask: &'static [u64],
}
