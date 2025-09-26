#[derive(Debug, Clone)]
pub struct ImplBuilder(crate::Impl);

impl ImplBuilder {
    pub fn new(path: &crate::Path, self_ty: &crate::Type) -> Self {
        return Self(crate::Impl {
            path: path.clone(),
            meta: crate::MetaData::new(),
            of_trait: None,
            self_ty: self_ty.clone(),
            methods: vec![],
        });
    }

    pub fn meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        return next;
    }

    pub fn of(&self, _trait: &crate::Path) -> Self {
        let mut next = self.clone();
        next.0.of_trait = Some(_trait.clone());
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

    pub fn build(&self) -> crate::Impl {
        return self.0.clone();
    }
}
