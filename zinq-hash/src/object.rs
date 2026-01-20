#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Object(blake3::Hash);

impl From<blake3::Hash> for Object {
    fn from(value: blake3::Hash) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Object {
    type Target = blake3::Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}
