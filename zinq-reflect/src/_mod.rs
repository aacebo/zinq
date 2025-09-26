#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) tys: Vec<crate::Type>,
}

impl ModType {
    pub fn new(path: &crate::Path) -> crate::build::ModTypeBuilder {
        return crate::build::ModTypeBuilder::new(path);
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(self.path.to_string());
    }

    pub fn path(&self) -> &crate::Path {
        return &self.path;
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn vis(&self) -> &crate::Visibility {
        return &self.vis;
    }

    pub fn tys(&self) -> &[crate::Type] {
        return &self.tys;
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Mod(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_mod();
    }
}

impl std::fmt::Display for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "{} {{", self.path.name())?;

        for ty in &self.tys {
            write!(f, "\n\t{}", ty)?;
        }

        if self.tys.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
