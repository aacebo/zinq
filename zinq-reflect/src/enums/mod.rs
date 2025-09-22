mod variant;

pub use variant::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumType {
    pub(crate) module: crate::Module,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) variants: Vec<Variant>,
}

impl EnumType {
    pub fn new(module: &crate::Module, name: &str) -> crate::build::EnumTypeBuilder {
        return crate::build::EnumTypeBuilder::new(module, name);
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str(&self.name);
    }

    pub fn len(&self) -> usize {
        return self.variants.len();
    }

    pub fn module(&self) -> &crate::Module {
        return &self.module;
    }

    pub fn vis(&self) -> &crate::Visibility {
        return &self.vis;
    }

    pub fn name(&self) -> &str {
        return &self.name;
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
        if self.vis != crate::Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    ty: EnumType,
    value: Box<crate::Value>,
}

impl Enum {
    pub fn new(ty: &EnumType, value: &crate::Value) -> Self {
        return Self {
            ty: ty.clone(),
            value: Box::new(value.clone()),
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Enum(self.ty.clone());
    }

    pub fn get(&self) -> &crate::Value {
        return &self.value;
    }
}

impl AsRef<crate::Value> for Enum {
    fn as_ref(&self) -> &crate::Value {
        return &self.value;
    }
}

impl AsMut<crate::Value> for Enum {
    fn as_mut(&mut self) -> &mut crate::Value {
        return &mut self.value;
    }
}

impl std::ops::Deref for Enum {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl std::ops::DerefMut for Enum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.value;
    }
}

impl crate::Reflect for Enum {
    fn reflect(self) -> crate::Value {
        return crate::Value::Enum(self.clone());
    }
}

impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.value);
    }
}
