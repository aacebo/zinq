use crate::id::attr::Attr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAttr {
    pub name: String,
    pub value: Box<Attr>,
}

impl std::fmt::Display for FieldAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}
