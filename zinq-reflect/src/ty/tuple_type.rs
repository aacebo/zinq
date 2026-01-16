use crate::{
    Path, Size,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(Vec<Type>);

impl ZinqType for TupleType {
    fn path(&self) -> Path {
        format!(
            "({})",
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into()
    }

    fn size(&self) -> Size {
        let mut size = 0;

        for ty in self.0.iter() {
            size += match ty.size() {
                Size::Dynamic => return Size::Dynamic,
                Size::Static(v) => v,
            };
        }

        Size::Static(size)
    }
}

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
