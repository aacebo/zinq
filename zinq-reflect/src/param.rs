use crate::ty::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.name, &self.ty)
    }
}
