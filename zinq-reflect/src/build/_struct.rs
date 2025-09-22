#[derive(Debug, Clone)]
pub struct StructTypeBuilder(crate::StructType);

impl StructTypeBuilder {
    pub fn new(module: &crate::Module, name: &str) -> Self {
        return Self(crate::StructType {
            module: module.clone(),
            vis: crate::Visibility::Private,
            name: name.to_string(),
            fields: super::FieldsBuilder::new().build(),
            methods: vec![],
        });
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        return next;
    }

    pub fn methods(&self, methods: &[crate::Method]) -> Self {
        let mut next = self.clone();
        next.0.methods = methods.to_vec();
        return next;
    }

    pub fn build(&self) -> crate::StructType {
        return self.0.clone();
    }
}
