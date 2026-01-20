use crate::{ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringType;

impl ZinqType for StringType {
    fn name(&self) -> String {
        "string".into()
    }
}

impl From<StringType> for Type {
    fn from(value: StringType) -> Self {
        Self::String(value)
    }
}

impl std::fmt::Display for StringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
