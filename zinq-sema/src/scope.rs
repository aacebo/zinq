use crate::ScopeId;

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
