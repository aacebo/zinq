use crate::{TypeId, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub item: TypeId,
    pub length: usize,
}

impl ZinqType for ArrayType {
    fn name(&self) -> String {
        format!("[{}; {}]", &self.item, &self.length)
    }

    fn refs(&self) -> Box<[TypeId]> {
        Box::new([self.item.clone()])
    }
}

impl From<ArrayType> for Type {
    fn from(value: ArrayType) -> Self {
        Self::Array(value)
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
