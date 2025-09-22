#[derive(Debug, Clone)]
pub struct FieldsBuilder(crate::Fields);

impl FieldsBuilder {
    pub fn new() -> Self {
        return Self(crate::Fields {
            layout: crate::Layout::Unit,
            fields: vec![],
        });
    }

    pub fn layout(&self, layout: crate::Layout) -> Self {
        let mut next = self.clone();
        next.0.layout = layout;
        return next;
    }

    pub fn fields(&self, fields: &[crate::Field]) -> Self {
        let mut next = self.clone();
        next.0.fields.append(&mut fields.to_vec());
        return next;
    }

    pub fn field(&self, field: &crate::Field) -> Self {
        let mut next = self.clone();
        next.0.fields.push(field.clone());
        return next;
    }

    pub fn build(&self) -> crate::Fields {
        return self.0.clone();
    }
}
