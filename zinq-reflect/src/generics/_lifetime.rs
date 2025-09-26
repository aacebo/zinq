#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LifetimeParam {
    pub(crate) name: String,
}

impl LifetimeParam {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }

    pub fn to_generic(&self) -> crate::Generic {
        return crate::Generic::Lifetime(self.clone());
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}

impl std::fmt::Display for LifetimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "'{}", &self.name);
    }
}
