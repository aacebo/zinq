use crate::{Param, Visibility};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Method {
    pub(crate) vis: Visibility,
    pub(crate) name: String,
    pub(crate) params: Vec<Param>,
    pub(crate) return_type: Option<Box<crate::Type>>,
}

impl Method {
    pub fn vis(&self) -> Visibility {
        return self.vis.clone();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn params(&self) -> &[Param] {
        return &self.params;
    }

    pub fn param(&self, name: &str) -> &Param {
        return self.params.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn has_param(&self, name: &str) -> bool {
        return self.params.iter().any(|v| v.name() == name);
    }

    pub fn param_mut(&mut self, name: &str) -> &mut Param {
        return self.params.iter_mut().find(|v| v.name() == name).unwrap();
    }

    pub fn return_type(&self) -> Option<&crate::Type> {
        return match &self.return_type {
            None => None,
            Some(v) => Some(&v),
        };
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "{}(", &self.name)?;

        for (i, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        if let Some(return_type) = &self.return_type {
            write!(f, " -> {}", return_type)?;
        }

        return write!(f, ";");
    }
}
