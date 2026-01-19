use crate::{Field, TypePath};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub path: TypePath,
    pub fields: Vec<Field>,
}

impl Variant {
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn field_at(&self, index: usize) -> &Field {
        &self.fields[index]
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
