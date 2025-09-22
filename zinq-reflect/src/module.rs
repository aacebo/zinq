#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module(String);

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl From<String> for Module {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.0);
    }
}
