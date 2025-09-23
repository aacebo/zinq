#[derive(Debug, Clone)]
pub struct MethodBuilder(crate::Method);

impl MethodBuilder {
    pub fn new(name: &str) -> Self {
        return Self(crate::Method {
            is_async: false,
            vis: crate::Visibility::Private,
            name: name.to_string(),
            params: vec![],
            return_type: Box::new(crate::Type::Void),
        });
    }

    pub fn is_async(&self, is_async: bool) -> Self {
        let mut next = self.clone();
        next.0.is_async = is_async;
        return next;
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn params(&self, params: &[crate::Param]) -> Self {
        let mut next = self.clone();
        next.0.params.append(&mut params.to_vec());
        return next;
    }

    pub fn param(&self, param: &crate::Param) -> Self {
        let mut next = self.clone();
        next.0.params.push(param.clone());
        return next;
    }

    pub fn return_type(&self, ty: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.return_type = Box::new(ty.clone());
        return next;
    }

    pub fn build(&self) -> crate::Method {
        return self.0.clone();
    }
}
