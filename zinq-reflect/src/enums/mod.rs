pub mod variants;

pub use variants::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumType {
    name: String,
    variants: Vec<Variant>,
}

impl EnumType {
    pub fn new(name: &str, variants: &[Variant]) -> Self {
        return Self {
            name: name.to_string(),
            variants: variants.to_vec(),
        };
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str(&self.name);
    }

    pub fn len(&self) -> usize {
        return self.variants.len();
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Enum(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_enum();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Variant> {
        return self.variants.iter();
    }

    pub fn has_variant(&self, name: &str) -> bool {
        return self.variants.iter().any(|v| v.name() == name);
    }

    pub fn variant(&self, name: &str) -> &Variant {
        return self.variants.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn variant_mut(&mut self, name: &str) -> &mut Variant {
        return self.variants.iter_mut().find(|v| v.name() == name).unwrap();
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "enum {} {{", &self.name)?;

        for variant in &self.variants {
            write!(f, "\n\t{}", variant)?;
        }

        if self.variants.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
