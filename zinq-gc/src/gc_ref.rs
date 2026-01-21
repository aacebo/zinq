use std::marker::PhantomData;

use crate::{GcAny, GcFooter, GcHeader};

///
/// ### GcRef
/// a reference to some GC managed object.
///
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GcRef<T> {
    __data__: PhantomData<*const T>,
    inner: GcAny,
}

impl<T> GcRef<T> {
    pub fn header(&self) -> &GcHeader {
        self.inner.header()
    }

    pub fn footer(&self) -> &GcFooter {
        self.inner.footer()
    }

    pub fn size(&self) -> usize {
        self.inner.size()
    }

    pub fn value(&self) -> &T {
        self.inner.value()
    }

    pub fn as_any(&self) -> &GcAny {
        &self.inner
    }
}
