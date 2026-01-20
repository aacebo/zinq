use crate::{TypeId, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(Vec<TypeId>);

impl TupleType {
    pub fn new(types: Vec<TypeId>) -> Self {
        Self(types)
    }
}

impl std::ops::Deref for TupleType {
    type Target = [TypeId];

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

    fn refs(&self) -> Box<[TypeId]> {
        self.0.clone().into_boxed_slice()
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
