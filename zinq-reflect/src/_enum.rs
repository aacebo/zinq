#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) variants: Vec<crate::Variant>,
}

impl EnumType {
    pub fn new(path: &crate::Path, name: &str) -> crate::build::EnumTypeBuilder {
        return crate::build::EnumTypeBuilder::new(path, name);
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Enum(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name));
    }

    pub fn len(&self) -> usize {
        return self.variants.len();
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

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn generics(&self) -> &crate::Generics {
        return &self.generics;
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_enum();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Variant> {
        return self.variants.iter();
    }

    pub fn has_variant(&self, name: &str) -> bool {
        return self.variants.iter().any(|v| v.name() == name);
    }

    pub fn variant(&self, name: &str) -> &crate::Variant {
        return self.variants.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn variant_mut(&mut self, name: &str) -> &mut crate::Variant {
        return self.variants.iter_mut().find(|v| v.name() == name).unwrap();
    }
}

impl crate::ToType for EnumType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Enum(self.clone());
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != crate::Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "enum {}{} {{", &self.name, &self.generics)?;

        for variant in &self.variants {
            write!(f, "\n\t{},", variant)?;
        }

        if self.variants.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
