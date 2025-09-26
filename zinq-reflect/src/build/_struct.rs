#[derive(Debug, Clone)]
pub struct StructTypeBuilder(crate::StructType);

impl StructTypeBuilder {
    pub fn new(path: &crate::Path, name: &str) -> Self {
        return Self(crate::StructType {
            path: path.clone(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: name.to_string(),
            generics: crate::Generics::new(),
            fields: super::FieldsBuilder::new().build(),
        });
    }

    pub fn meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        return next;
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn generics(&self, generics: &crate::Generics) -> Self {
        let mut next = self.clone();
        next.0.generics = generics.clone();
        return next;
    }

    pub fn fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        return next;
    }

    pub fn build(&self) -> crate::StructType {
        return self.0.clone();
    }
}
