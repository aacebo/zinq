use crate::{
    Size,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(Vec<Type>);

impl TupleType {
    pub fn new(types: Vec<Type>) -> Self {
        Self(types)
    }
}

impl std::ops::Deref for TupleType {
    type Target = [Type];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ZinqType for TupleType {
    fn name(&self) -> String {
        format!(
            "({})",
            self.0
                .iter()
                .map(|t| t.to_string())
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

impl From<TupleType> for Type {
    fn from(value: TupleType) -> Self {
        Self::Tuple(value)
    }
}

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
