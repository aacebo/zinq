use crate::{
    Size, TypePtr,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PtrType {
    pub ty: TypePtr,
}

impl ZinqType for PtrType {
    fn name(&self) -> String {
        format!("&{}", &self.ty).into()
    }

    fn size(&self) -> Size {
        // 32 bit
        if cfg!(target_arch = "x86") {
            return Size::Static(4);
        }

        // 64 bit
        Size::Static(8)
    }

    fn refs(&self) -> Box<[TypePtr]> {
        vec![self.ty.clone()].into_boxed_slice()
    }
}

impl From<TypePtr> for PtrType {
    fn from(ty: TypePtr) -> Self {
        Self { ty }
    }
}

impl From<Type> for PtrType {
    fn from(value: Type) -> Self {
        Self { ty: value.ptr() }
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
