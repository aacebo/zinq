use crate::Visibility;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    visibility: Visibility,
    name: String,
    _type: Box<crate::Type>,
}

impl Field {
    pub fn vis(&self) -> Visibility {
        return self.visibility.clone();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn ty(&self) -> &crate::Type {
        return &self._type;
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} {}: {},", &self.visibility, &self.name, &self._type);
    }
}
