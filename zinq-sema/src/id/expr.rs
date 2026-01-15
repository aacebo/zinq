use crate::id::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(blake3::Hash);

impl ExprId {
    pub fn new(name: &str) -> Builder {
        Builder::new().attr(name.into())
    }
}

impl From<blake3::Hash> for ExprId {
    fn from(value: blake3::Hash) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}

impl std::ops::Deref for ExprId {
    type Target = blake3::Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
