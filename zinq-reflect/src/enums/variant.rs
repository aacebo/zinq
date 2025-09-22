#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    pub(crate) name: String,
    pub(crate) fields: crate::Fields,
}

impl Variant {
    pub fn new(name: &str) -> crate::build::VariantBuilder {
        return crate::build::VariantBuilder::new(name);
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn fields(&self) -> &crate::Fields {
        return &self.fields;
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}{}", &self.name, &self.fields);
    }
}
