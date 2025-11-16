#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) fields: crate::Fields,
}

impl StructType {
    pub fn new(path: &crate::Path, name: &str) -> crate::build::StructTypeBuilder {
        return crate::build::StructTypeBuilder::new(path, name);
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name));
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
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

    pub fn fields(&self) -> &crate::Fields {
        return &self.fields;
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_struct();
    }
}

impl crate::ToType for StructType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.clone());
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != crate::Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        return write!(f, "struct {}{}{}", &self.name, &self.generics, &self.fields);
    }
}
