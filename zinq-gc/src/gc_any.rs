use std::ptr::NonNull;

use crate::{GcFooter, GcHeader};

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

    pub fn footer(&self) -> &GcFooter {
        let base = self.0.as_ptr() as *const u8;

        unsafe {
            let ptr = base.add(self.size() - size_of::<GcFooter>()) as *const GcFooter;

            match ptr.as_ref() {
                None => &GcFooter::Fixed,
                Some(v) => v,
            }
        }
    }

    pub fn size(&self) -> usize {
        (*self.header().size()) as usize
    }

    pub fn value<T>(&self) -> &T {
        unsafe { self.0.cast().as_ref() }
    }
}
