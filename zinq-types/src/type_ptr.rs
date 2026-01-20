use crate::{Size, TypeId, TypePath, ZinqType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePtr {
    pub id: TypeId,
    pub path: TypePath,
    pub size: Size,
}

impl<T: ZinqType> From<&T> for TypePtr {
    fn from(ty: &T) -> Self {
        let path = match ty.module() {
            None => ty.name(),
            Some(v) => format!("{}::{}", &v, ty.name()),
        };

        Self {
            id: ty.id(),
            path: TypePath::from(path),
            size: ty.size(),
        }
    }
}

impl std::fmt::Display for TypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
