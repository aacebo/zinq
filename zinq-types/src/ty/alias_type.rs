use crate::{Path, TypeId, TypePath, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AliasType {
    pub path: TypePath,
    pub ty: TypeId,
}

impl ZinqType for AliasType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn refs(&self) -> Box<[TypeId]> {
        Box::new([self.ty.clone()])
    }
}

impl From<AliasType> for Type {
    fn from(value: AliasType) -> Self {
        Self::Alias(value)
    }
}

impl std::fmt::Display for AliasType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
