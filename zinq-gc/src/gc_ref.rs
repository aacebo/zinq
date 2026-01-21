use std::{marker::PhantomData, ptr::NonNull};

use crate::GcMeta;

///
/// ### GcRef
/// a reference to some `GcPtr<T>` object
///
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GcRef<T> {
    __data__: PhantomData<T>,
    ptr: NonNull<GcMeta>,
}
