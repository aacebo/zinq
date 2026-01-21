use std::ptr::NonNull;

use crate::GcHeader;

///
/// ### GcAny
/// a reference to some object
///
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GcAny(NonNull<GcHeader>);

impl GcAny {
    pub fn header(&self) -> &GcHeader {
        unsafe { self.0.as_ref() }
    }

    pub fn value<T>(&self) -> &T {
        unsafe { self.0.cast().as_ref() }
    }

    pub fn value_ptr<T>(&self) -> *mut T {
        self.0.cast().as_ptr()
    }
}
