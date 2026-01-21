use std::marker::PhantomData;

use crate::GcAny;

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
    pub fn value(&self) -> &T {
        self.inner.value()
    }

    pub fn as_any(&self) -> &GcAny {
        &self.inner
    }
}

impl<T> AsRef<GcAny> for GcRef<T> {
    fn as_ref(&self) -> &GcAny {
        &self.inner
    }
}
