pub mod arena;
mod gc_any;
mod gc_footer;
mod gc_header;
mod gc_mark;
mod gc_ref;

pub use gc_any::*;
pub use gc_footer::*;
pub use gc_header::*;
pub use gc_mark::*;
pub use gc_ref::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId {
    pub fn as_u32(&self) -> &u32 {
        &self.0
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

impl std::ops::Deref for ObjectId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
