#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) default: Option<crate::Type>,
    pub(crate) bounds: Vec<crate::Bound>,
}

impl TypeParam {
    pub fn to_generic(&self) -> crate::Generic {
        return crate::Generic::Type(self.clone());
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn bounds(&self) -> &[crate::Bound] {
        return &self.bounds;
    }

    pub fn default(&self) -> Option<&crate::Type> {
        return match &self.default {
            None => None,
            Some(v) => Some(v),
        };
    }
}

impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)?;

        if self.bounds.len() > 0 {
            write!(f, ": ")?;
        }

        for (i, bound) in self.bounds.iter().enumerate() {
            write!(f, "{}", bound)?;

            if i < self.bounds.len() - 1 {
                write!(f, " + ")?;
            }
        }

        if let Some(default) = &self.default {
            write!(f, " = {}", default)?;
        }

        return Ok(());
    }
}
