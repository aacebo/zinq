use crate::{
    ty::{PtrType, Type},
    value::{RefId, Value, ZinqValue},
};

#[derive(Debug, Clone, PartialEq)]
pub struct RefValue {
    pub id: RefId,
    pub ty: Type,
}

impl From<RefValue> for Value {
    fn from(value: RefValue) -> Self {
        Self::Ref(value)
    }
}

impl std::fmt::Display for RefValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id)
    }
}

impl ZinqValue for RefValue {
    fn ty(&self) -> Type {
        PtrType {
            ty: Box::new(self.ty.clone()),
        }
        .into()
    }
}
