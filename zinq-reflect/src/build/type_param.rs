#[derive(Debug, Clone)]
pub struct TypeParamBuilder(crate::TypeParam);

impl TypeParamBuilder {
    pub fn new(name: &str) -> Self {
        return Self(crate::TypeParam {
            name: name.to_string(),
            default: None,
            bounds: vec![],
        });
    }

    pub fn default(&self, default: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.default = Some(default.clone());
        return next;
    }

    pub fn bounds(&self, bounds: &[crate::Bound]) -> Self {
        let mut next = self.clone();
        next.0.bounds.append(&mut bounds.to_vec());
        return next;
    }

    pub fn bound(&self, bound: &crate::Bound) -> Self {
        let mut next = self.clone();
        next.0.bounds.push(bound.clone());
        return next;
    }

    pub fn build(&self) -> crate::TypeParam {
        return self.0.clone();
    }
}
