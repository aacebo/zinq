#[derive(Debug, Clone)]
pub struct VariantBuilder(crate::Variant);

impl VariantBuilder {
    pub fn new(name: &str) -> Self {
        return Self(crate::Variant {
            meta: crate::MetaData::new(),
            name: name.to_string(),
            fields: super::FieldsBuilder::new().build(),
        });
    }

    pub fn meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        return next;
    }

    pub fn fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        return next;
    }

    pub fn build(&self) -> crate::Variant {
        return self.0.clone();
    }
}
