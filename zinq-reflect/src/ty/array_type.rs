use crate::{
    Size, TypePtr,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub item: TypePtr,
    pub length: usize,
}

impl ZinqType for ArrayType {
    fn name(&self) -> String {
        format!("[{}; {}]", &self.item, &self.length)
    }

    fn size(&self) -> Size {
        match self.item.size {
            Size::Dynamic => Size::Dynamic,
            Size::Static(v) => Size::Static(v * self.length),
        }
    }

    fn refs(&self) -> Box<[TypePtr]> {
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
