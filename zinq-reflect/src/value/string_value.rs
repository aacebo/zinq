use crate::{
    ty::{StringType, Type},
    value::{ObjectValue, ZinqValue},
};

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue(String);

impl From<StringValue> for ObjectValue {
    fn from(value: StringValue) -> Self {
        Self::String(value)
    }
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StringValue {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::ops::Deref for StringValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ZinqValue for StringValue {
    fn ty(&self) -> Type {
        StringType.into()
    }
}
