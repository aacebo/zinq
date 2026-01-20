use crate::{Size, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolType;

impl ZinqType for BoolType {
    fn name(&self) -> String {
        "bool".into()
    }

    fn size(&self) -> Size {
        Size::Static(1)
    }
}

impl From<BoolType> for Type {
    fn from(value: BoolType) -> Self {
        Self::Bool(value)
    }
}

impl std::fmt::Display for BoolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
