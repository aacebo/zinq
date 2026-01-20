mod value;

pub use value::*;

pub trait ZinqValue {
    fn ty(&self) -> zinq_types::TypeId;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValueId(u32);

impl ValueId {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for ValueId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ValueId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ValueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
