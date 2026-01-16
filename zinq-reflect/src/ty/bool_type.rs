use crate::{Path, Size, ty::ZinqType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolType;

impl ZinqType for BoolType {
    fn path(&self) -> Path {
        "bool".into()
    }

    fn size(&self) -> Size {
        Size::Static(1)
    }
}

impl std::fmt::Display for BoolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
