use crate::{Impl, Path, Size, TypePath, TypePtr, Variant, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumType {
    pub path: TypePath,
    pub variants: Vec<Variant>,
    pub impls: Vec<Impl>,
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
                size += match field.ty.size {
                    Size::Dynamic => return Size::Dynamic,
                    Size::Static(v) => v,
                };
            }
        }

        Size::Static(size)
    }

    fn refs(&self) -> Box<[TypePtr]> {
        let variants = self
            .variants
            .iter()
            .flat_map(|v| v.fields.iter().map(|f| f.ty.clone()))
            .collect::<Vec<_>>();
        let impls = self
            .impls
            .iter()
            .flat_map(|im| im.refs())
            .collect::<Vec<_>>();
        vec![variants, impls].concat().into_boxed_slice()
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
