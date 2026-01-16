use crate::{Path, Size, ty::ZinqType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PtrType {
    pub inner: Path,
}

impl ZinqType for PtrType {
    fn path(&self) -> Path {
        format!("ptr<{}>", &self.inner).into()
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

impl std::fmt::Display for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
