#[derive(Debug, Clone)]
pub struct FieldBuilder(crate::Field);

impl FieldBuilder {
    pub fn new(name: &crate::FieldName, ty: &crate::Type) -> Self {
        return Self(crate::Field {
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: name.clone(),
            ty: Box::new(ty.clone()),
        });
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn build(&self) -> crate::Field {
        return self.0.clone();
    }
}
