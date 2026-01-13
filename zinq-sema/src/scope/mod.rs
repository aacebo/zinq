#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u32);

impl From<u32> for ScopeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ScopeId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ScopeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    id: ScopeId,
    parent: Option<Box<Self>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            id: ScopeId::default(),
            parent: None,
        }
    }

    pub fn branch(&self) -> Self {
        Self {
            id: ScopeId(*self.id + 1),
            parent: Some(Box::new(self.clone())),
        }
    }
}
