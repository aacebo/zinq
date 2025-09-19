#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitVariant {
    name: String,
}

impl UnitVariant {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}

impl std::fmt::Display for UnitVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.name);
    }
}
