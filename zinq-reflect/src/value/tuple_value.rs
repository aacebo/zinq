use crate::{
    ty::{TupleType, Type, ZinqType},
    value::{Value, ZinqValue},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TupleValue(Vec<Value>);

impl std::ops::Deref for TupleValue {
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ZinqValue for TupleValue {
    fn ty(&self) -> Type {
        TupleType::new(self.0.iter().map(|v| v.ty().ptr()).collect::<Vec<_>>()).into()
    }
}

impl std::fmt::Display for TupleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for (i, value) in self.0.iter().enumerate() {
            write!(f, "{}", value)?;

            if i < self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}
