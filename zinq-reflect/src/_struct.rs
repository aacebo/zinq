use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Struct {
    ty: StructType,
    fields: BTreeMap<String, crate::Value>,
}

impl Struct {
    pub fn new(ty: &StructType) -> Self {
        let mut fields = BTreeMap::new();

        for field in ty.fields().iter() {
            fields.insert(field.name().to_string(), crate::Value::Null);
        }

        return Self {
            ty: ty.clone(),
            fields,
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.ty.clone());
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, crate::Value> {
        return self.fields.iter();
    }

    pub fn get(&self, name: &str) -> Option<&crate::Value> {
        return self.fields.get(name);
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut crate::Value> {
        return self.fields.get_mut(name);
    }
}

impl crate::ToType for Struct {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.ty.clone());
    }
}

impl crate::ToValue for Struct {
    fn to_value(self) -> crate::Value {
        return crate::Value::Struct(self.clone());
    }
}

impl Index<&str> for Struct {
    type Output = crate::Value;

    fn index(&self, index: &str) -> &Self::Output {
        return self.fields.index(index);
    }
}

impl IndexMut<&str> for Struct {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return self.fields.get_mut(index).unwrap();
    }
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.fields {
            write!(f, "\n\t{}: {}", key, value)?;
        }

        if self.fields.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
