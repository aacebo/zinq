use crate::{
    Param, Path, Size, TypePath, TypePtr,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType {
    pub path: TypePath,
    pub params: Vec<Param>,
    pub return_ty: Option<TypePtr>,
}

impl ZinqType for FnType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn size(&self) -> Size {
        Size::Static(0)
    }
}

impl From<FnType> for Type {
    fn from(value: FnType) -> Self {
        Self::Fn(value)
    }
}

impl std::fmt::Display for FnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
