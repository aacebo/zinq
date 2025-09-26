#[derive(Debug, Clone)]
pub struct EnumTypeBuilder(crate::EnumType);

impl EnumTypeBuilder {
    pub fn new(path: &crate::Path, name: &str) -> Self {
        return Self(crate::EnumType {
            path: path.clone(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: name.to_string(),
            variants: vec![],
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

    pub fn variants(&self, variants: &[crate::Variant]) -> Self {
        let mut next = self.clone();
        next.0.variants.append(&mut variants.to_vec());
        return next;
    }

    pub fn variant(&self, variant: &crate::Variant) -> Self {
        let mut next = self.clone();
        next.0.variants.push(variant.clone());
        return next;
    }

    pub fn build(&self) -> crate::EnumType {
        return self.0.clone();
    }
}
