#[derive(Debug, Clone)]
pub struct TraitTypeBuilder(crate::TraitType);

impl TraitTypeBuilder {
    pub fn new(path: &crate::Path, name: &str) -> Self {
        return Self(crate::TraitType {
            path: path.clone(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: name.to_string(),
            generics: crate::Generics::new(),
            methods: vec![],
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

    pub fn methods(&self, methods: &[crate::Method]) -> Self {
        let mut next = self.clone();
        next.0.methods.append(&mut methods.to_vec());
        return next;
    }

    pub fn method(&self, method: &crate::Method) -> Self {
        let mut next = self.clone();
        next.0.methods.push(method.clone());
        return next;
    }

    pub fn build(&self) -> crate::TraitType {
        return self.0.clone();
    }
}
