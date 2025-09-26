use crate::{Param, Visibility};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Method {
    pub(crate) meta: crate::MetaData,
    pub(crate) is_async: bool,
    pub(crate) vis: Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) params: Vec<Param>,
    pub(crate) return_type: Box<crate::Type>,
}

impl Method {
    pub fn new(name: &str) -> crate::build::MethodBuilder {
        return crate::build::MethodBuilder::new(name);
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn is_async(&self) -> bool {
        return self.is_async;
    }

    pub fn vis(&self) -> Visibility {
        return self.vis.clone();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn generics(&self) -> &crate::Generics {
        return &self.generics;
    }

    pub fn params(&self) -> &[Param] {
        return &self.params;
    }

    pub fn has_param(&self, name: &str) -> bool {
        return self.params.iter().any(|v| v.name() == name);
    }

    pub fn param(&self, name: &str) -> &Param {
        return self.params.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn param_mut(&mut self, name: &str) -> &mut Param {
        return self.params.iter_mut().find(|v| v.name() == name).unwrap();
    }

    pub fn return_type(&self) -> &crate::Type {
        return &self.return_type;
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        if self.is_async {
            write!(f, "async ")?;
        }

        write!(f, "fn {}{}(", &self.name, &self.generics)?;

        for (i, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        if !self.return_type.is_void() {
            write!(f, " -> {}", &self.return_type)?;
        }

        return write!(f, ";");
    }
}
