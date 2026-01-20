use crate::{Param, Path, Size, TypeId, TypePath, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType {
    pub path: TypePath,
    pub params: Vec<Param>,
    pub return_ty: Option<TypeId>,
}

impl ZinqType for FnType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn size(&self) -> Size {
        Size::Known(0)
    }

    fn refs(&self) -> Box<[TypeId]> {
        let params = self.params.iter().map(|p| p.ty.clone()).collect::<Vec<_>>();

        if let Some(ty) = &self.return_ty {
            return [params, vec![ty.clone()]].concat().into_boxed_slice();
        }

        params.into_boxed_slice()
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
