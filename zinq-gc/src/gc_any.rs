use std::ptr::NonNull;

use crate::GcMeta;

///
/// ### GcAny
/// a reference to some object
///
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GcAny(NonNull<GcMeta>);
