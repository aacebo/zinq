#[derive(Debug, Clone)]
pub struct TraitTypeBuilder(crate::TraitType);

impl TraitTypeBuilder {
    pub fn new(module: &crate::Module, name: &str) -> Self {
        return Self(crate::TraitType {
            module: module.clone(),
            vis: crate::Visibility::Private,
            name: name.to_string(),
            methods: vec![],
        });
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
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
