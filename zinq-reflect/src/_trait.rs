#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) methods: Vec<crate::Method>,
}

impl TraitType {
    pub fn new(path: &crate::Path, name: &str) -> crate::build::TraitTypeBuilder {
        return crate::build::TraitTypeBuilder::new(path, name);
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Trait(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_trait();
    }
}

impl TraitType {
    pub fn path(&self) -> &crate::Path {
        return &self.path;
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn vis(&self) -> &crate::Visibility {
        return &self.vis;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn methods(&self) -> &[crate::Method] {
        return &self.methods;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Method> {
        return self.methods.iter();
    }

    pub fn len(&self) -> usize {
        return self.methods.len();
    }

    pub fn has(&self, name: &str) -> bool {
        return self.methods.iter().any(|v| v.name() == name);
    }

    pub fn get(&self, name: &str) -> Option<&crate::Method> {
        return self.methods.iter().find(|v| v.name() == name);
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut crate::Method> {
        return self.methods.iter_mut().find(|v| v.name() == name);
    }
}

impl AsRef<TraitType> for TraitType {
    fn as_ref(&self) -> &Self {
        return self;
    }
}

impl AsMut<TraitType> for TraitType {
    fn as_mut(&mut self) -> &mut Self {
        return self;
    }
}

impl std::ops::Index<usize> for TraitType {
    type Output = crate::Method;

    fn index(&self, index: usize) -> &Self::Output {
        return self.methods.index(index);
    }
}

impl std::ops::IndexMut<usize> for TraitType {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.methods.index_mut(index);
    }
}

impl std::ops::Index<&str> for TraitType {
    type Output = crate::Method;

    fn index(&self, index: &str) -> &Self::Output {
        return self.get(index).unwrap();
    }
}

impl std::ops::IndexMut<&str> for TraitType {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return self.get_mut(index).unwrap();
    }
}

impl std::fmt::Display for TraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "trait {} {{", &self.name)?;

        for method in &self.methods {
            write!(f, "\n\t{}", method)?;
        }

        if self.methods.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
