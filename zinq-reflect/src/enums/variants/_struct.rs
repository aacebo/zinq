#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructVariant {
    name: String,
    fields: Vec<crate::Field>,
}

impl StructVariant {
    pub fn new(name: &str, fields: &[crate::Field]) -> Self {
        return Self {
            name: name.to_string(),
            fields: fields.to_vec(),
        };
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Field> {
        return self.fields.iter();
    }

    pub fn has_field(&self, name: &str) -> bool {
        return self.fields.iter().any(|v| v.name() == name);
    }

    pub fn field(&self, name: &str) -> &crate::Field {
        return self.fields.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn field_mut(&mut self, name: &str) -> &mut crate::Field {
        return self.fields.iter_mut().find(|v| v.name() == name).unwrap();
    }
}

impl std::fmt::Display for StructVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{", &self.name)?;

        for field in &self.fields {
            write!(f, "\n\t{}", field)?;
        }

        if self.fields.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
