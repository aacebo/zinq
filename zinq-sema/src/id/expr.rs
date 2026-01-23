#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(zinq_hash::Object);

impl ExprId {
    pub fn new(name: &str) -> zinq_hash::Object {
        zinq_hash::v1().push_str(name).build()
    }
}

impl From<zinq_hash::Object> for ExprId {
    fn from(value: zinq_hash::Object) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}

impl std::ops::Deref for ExprId {
    type Target = zinq_hash::Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
