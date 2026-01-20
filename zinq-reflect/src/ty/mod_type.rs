use crate::{
    Path, Size, TypePath, TypePtr,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModType {
    pub path: TypePath,
    pub types: Vec<TypePtr>,
}

impl ZinqType for ModType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn size(&self) -> Size {
        let mut size = 0;

        for ty in self.types.iter() {
            size += match ty.size {
                Size::Dynamic => return Size::Dynamic,
                Size::Static(v) => v,
            };
        }

        Size::Static(size)
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
