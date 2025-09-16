#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeId(String);

impl TypeId {
    pub(crate) fn from_str(value: &str) -> Self {
        return Self(value.to_string());
    }

    pub(crate) fn from_string(value: String) -> Self {
        return Self(value);
    }
}

impl std::fmt::Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.0);
    }
}

impl Eq for TypeId {}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl PartialEq<&str> for TypeId {
    fn eq(&self, other: &&str) -> bool {
        return &self.0 == other;
    }
}

impl PartialEq<String> for TypeId {
    fn eq(&self, other: &String) -> bool {
        return &self.0 == other;
    }
}
