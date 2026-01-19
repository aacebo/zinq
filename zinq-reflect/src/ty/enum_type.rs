use crate::{
    Path, Size, TypePath, Variant,
    ty::{Type, ZinqType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumType {
    pub path: TypePath,
    pub variants: Vec<Variant>,
}

impl EnumType {
    pub fn variant(&self, name: &str) -> Option<&Variant> {
        self.variants.iter().find(|v| v.path.ident == name)
    }

    pub fn variant_at(&self, index: usize) -> &Variant {
        &self.variants[index]
    }
}

impl ZinqType for EnumType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn size(&self) -> Size {
        let mut size = 0;

        for variant in self.variants.iter() {
            for field in &variant.fields {
                size += match field.ty.size() {
                    Size::Dynamic => return Size::Dynamic,
                    Size::Static(v) => v,
                };
            }
        }

        Size::Static(size)
    }
}

impl From<EnumType> for Type {
    fn from(value: EnumType) -> Self {
        Self::Enum(value)
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
