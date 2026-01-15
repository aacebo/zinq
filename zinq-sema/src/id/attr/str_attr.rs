use crate::id::Attr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrAttr(String);

impl From<&str> for StrAttr {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for StrAttr {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Attr {
    fn from(value: &str) -> Self {
        Self::Str(value.into())
    }
}

impl From<String> for Attr {
    fn from(value: String) -> Self {
        Self::Str(value.into())
    }
}

impl std::ops::Deref for StrAttr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for StrAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
