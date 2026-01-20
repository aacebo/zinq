use crate::{Size, TypeId, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PtrType {
    pub ty: TypeId,
}

impl ZinqType for PtrType {
    fn name(&self) -> String {
        format!("&{}", &self.ty).into()
    }

    fn size(&self) -> Size {
        // x86 = 4, x64 = 8
        Size::Known(size_of::<usize>())
    }

    fn refs(&self) -> Box<[TypeId]> {
        vec![self.ty].into_boxed_slice()
    }
}

impl From<TypeId> for PtrType {
    fn from(ty: TypeId) -> Self {
        Self { ty }
    }
}

impl From<Type> for PtrType {
    fn from(value: Type) -> Self {
        Self { ty: value.id() }
    }
}

impl From<PtrType> for Type {
    fn from(value: PtrType) -> Self {
        Self::Ptr(value)
    }
}

impl std::fmt::Display for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
