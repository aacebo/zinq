use crate::{Path, Size, ty::ZinqType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringType;

impl ZinqType for StringType {
    fn path(&self) -> Path {
        "string".into()
    }

    fn size(&self) -> Size {
        Size::Dynamic
    }
}

impl std::fmt::Display for StringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
