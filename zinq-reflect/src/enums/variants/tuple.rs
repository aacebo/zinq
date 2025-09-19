#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleVariant {
    name: String,
    types: Vec<crate::Type>,
}

impl TupleVariant {
    pub fn new(name: &str, types: &[crate::Type]) -> Self {
        return Self {
            name: name.to_string(),
            types: types.to_vec(),
        };
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn len(&self) -> usize {
        return self.types.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Type> {
        return self.types.iter();
    }

    pub fn has_ty(&self, id: crate::TypeId) -> bool {
        return self.types.iter().any(|v| v.id() == id);
    }

    pub fn ty(&self, id: crate::TypeId) -> &crate::Type {
        return self.types.iter().find(|v| v.id() == id).unwrap();
    }

    pub fn ty_mut(&mut self, id: crate::TypeId) -> &mut crate::Type {
        return self.types.iter_mut().find(|v| v.id() == id).unwrap();
    }
}

impl std::fmt::Display for TupleVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (", &self.name)?;

        for (i, ty) in self.types.iter().enumerate() {
            write!(f, "{}", ty)?;

            if i < self.types.len() - 1 {
                write!(f, ", ")?;
            }
        }

        return write!(f, ")");
    }
}
