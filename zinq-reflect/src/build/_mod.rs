#[derive(Debug, Clone)]
pub struct ModTypeBuilder(crate::ModType);

impl ModTypeBuilder {
    pub fn new(path: &crate::Path) -> Self {
        return Self(crate::ModType {
            path: path.clone(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            tys: vec![],
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

    pub fn types(&self, tys: &[crate::Type]) -> Self {
        let mut next = self.clone();
        next.0.tys.append(&mut tys.to_vec());
        return next;
    }

    pub fn build(&self) -> crate::ModType {
        return self.0.clone();
    }
}
