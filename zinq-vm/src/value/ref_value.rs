use zinq_types::{TypePtr, ZinqType, ty::PtrType};

use crate::value::{Value, ValueId, ZinqValue};

#[derive(Debug, Clone, PartialEq)]
pub struct RefValue {
    pub id: ValueId,
    pub ty: TypePtr,
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
    fn ty(&self) -> TypePtr {
        PtrType {
            ty: self.ty.clone(),
        }
        .ptr()
    }
}
