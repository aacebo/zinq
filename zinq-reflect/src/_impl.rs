#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Impl {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) of_trait: Option<crate::Path>,
    pub(crate) self_ty: crate::Type,
    pub(crate) methods: Vec<crate::Method>,
}

impl Impl {
    pub fn new(path: &crate::Path, self_ty: &crate::Type) -> crate::build::ImplBuilder {
        return crate::build::ImplBuilder::new(path, self_ty);
    }

    pub fn to_item(&self) -> crate::Item {
        return crate::Item::Impl(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        let mut path = self.path.clone() + self.self_ty.path();

        if let Some(of) = &self.of_trait {
            path = path + of;
        }

        return crate::TypeId::from_string(path.to_string());
    }

    pub fn len(&self) -> usize {
        return self.methods.len();
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn of_trait(&self) -> Option<&crate::Path> {
        return match &self.of_trait {
            None => None,
            Some(v) => Some(v),
        };
    }

    pub fn self_ty(&self) -> &crate::Type {
        return &self.self_ty;
    }

    pub fn methods(&self) -> &[crate::Method] {
        return &self.methods;
    }

    pub fn has_method(&self, name: &str) -> bool {
        return self.methods.iter().any(|v| v.name() == name);
    }

    pub fn method(&self, name: &str) -> &crate::Method {
        return self.methods.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn method_mut(&mut self, name: &str) -> &mut crate::Method {
        return self.methods.iter_mut().find(|v| v.name() == name).unwrap();
    }
}

impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impl ")?;

        if let Some(of) = &self.of_trait {
            write!(f, "{} for ", of.name())?;
        }

        write!(f, "{} {{", &self.self_ty)?;

        for method in &self.methods {
            write!(f, "\n\t{}", method)?;
        }

        if self.methods.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
