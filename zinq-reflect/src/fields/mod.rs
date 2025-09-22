mod field;

pub use field::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fields {
    pub(crate) layout: crate::Layout,
    pub(crate) fields: Vec<Field>,
}

impl Fields {
    pub fn new() -> crate::build::FieldsBuilder {
        return crate::build::FieldsBuilder::new();
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Field> {
        return self.fields.iter();
    }

    pub fn has(&self, name: &FieldName) -> bool {
        return self.fields.iter().any(|v| v.name() == name);
    }

    pub fn get(&self, name: &FieldName) -> Option<&Field> {
        return self.fields.iter().find(|v| v.name() == name);
    }

    pub fn get_mut(&mut self, name: &FieldName) -> Option<&mut Field> {
        return self.fields.iter_mut().find(|v| v.name() == name);
    }
}

impl From<&[crate::Field]> for Fields {
    fn from(value: &[crate::Field]) -> Self {
        return Self::new().fields(value).build();
    }
}

impl<const N: usize> From<&[crate::Field; N]> for Fields {
    fn from(value: &[crate::Field; N]) -> Self {
        return Self::new().fields(value).build();
    }
}

impl<const N: usize> From<[crate::Field; N]> for Fields {
    fn from(value: [crate::Field; N]) -> Self {
        return Self::new().fields(&value).build();
    }
}

impl AsRef<Fields> for Fields {
    fn as_ref(&self) -> &Self {
        return self;
    }
}

impl AsMut<Fields> for Fields {
    fn as_mut(&mut self) -> &mut Self {
        return self;
    }
}

impl std::ops::Index<usize> for Fields {
    type Output = Field;

    fn index(&self, index: usize) -> &Self::Output {
        return self.get(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::IndexMut<usize> for Fields {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::Index<&str> for Fields {
    type Output = Field;

    fn index(&self, index: &str) -> &Self::Output {
        return self.get(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::IndexMut<&str> for Fields {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return self.get_mut(&FieldName::from(index)).unwrap();
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.layout.is_key() {
            write!(f, " {{")?;

            for field in &self.fields {
                write!(f, "\n\t{},", field)?;
            }

            if self.fields.len() > 0 {
                write!(f, "\n")?;
            }

            return write!(f, "}}");
        }

        if self.layout.is_index() {
            write!(f, "(")?;

            for (i, field) in self.fields.iter().enumerate() {
                write!(f, "{}", field.ty())?;

                if i < self.fields.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            return write!(f, ")");
        }

        return Ok(());
    }
}
