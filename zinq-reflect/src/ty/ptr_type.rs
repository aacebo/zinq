use crate::{
    Size,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PtrType {
    pub ty: Box<Type>,
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
