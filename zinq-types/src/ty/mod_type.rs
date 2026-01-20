use crate::{Path, TypeId, TypePath, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModType {
    pub path: TypePath,
    pub types: Vec<TypeId>,
}

impl ZinqType for ModType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn refs(&self) -> Box<[TypeId]> {
        self.types.clone().into_boxed_slice()
    }
}

impl From<ModType> for Type {
    fn from(value: ModType) -> Self {
        Self::Mod(value)
    }
}

impl std::fmt::Display for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
