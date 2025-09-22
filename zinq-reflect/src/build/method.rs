#[derive(Debug, Clone)]
pub struct MethodBuilder(crate::Method);

impl MethodBuilder {
    pub fn new(name: &str) -> Self {
        return Self(crate::Method {
            vis: crate::Visibility::Private,
            name: name.to_string(),
            params: vec![],
            return_type: None,
        });
    }

    pub fn visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn params(&self, params: &[(&str, &crate::Type)]) -> Self {
        let mut next = self.clone();

        for (name, ty) in params {
            next = next.param(name, ty);
        }

        return next;
    }

    pub fn param(&self, name: &str, ty: &crate::Type) -> Self {
        let mut next = self.clone();

        next.0.params.push(crate::Param {
            name: name.to_string(),
            ty: Box::new(ty.clone()),
        });

        return next;
    }

    pub fn return_type(&self, ty: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.return_type = Some(Box::new(ty.clone()));
        return next;
    }

    pub fn build(&self) -> crate::Method {
        return self.0.clone();
    }
}
