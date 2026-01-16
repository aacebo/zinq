use crate::{
    Path, Size,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub item: Box<Type>,
    pub length: usize,
}

impl ZinqType for ArrayType {
    fn path(&self) -> Path {
        format!("[{}; {}]", &self.item, &self.length).into()
    }

    fn size(&self) -> Size {
        match self.item.size() {
            Size::Dynamic => Size::Dynamic,
            Size::Static(v) => Size::Static(v * self.length),
        }
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
