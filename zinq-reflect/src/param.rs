#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    pub(crate) name: String,
    pub(crate) ty: Box<crate::Type>,
}

impl Param {
    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}: {}", &self.name, &self.ty);
    }
}
