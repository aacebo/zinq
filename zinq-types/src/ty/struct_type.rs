use crate::{Field, Impl, Path, Size, TypePath, TypePtr, ZinqType, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
    pub path: TypePath,
    pub fields: Vec<Field>,
    pub impls: Vec<Impl>,
}

impl StructType {
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn field_at(&self, index: usize) -> &Field {
        &self.fields[index]
    }
}

impl ZinqType for StructType {
    fn name(&self) -> String {
        self.path.ident.clone()
    }

    fn module(&self) -> Option<Path> {
        Some(self.path.module.clone())
    }

    fn size(&self) -> Size {
        let mut size = 0;

        for field in self.fields.iter() {
            size += match field.ty.size {
                Size::Dynamic => return Size::Dynamic,
                Size::Static(v) => v,
            };
        }

        Size::Static(size)
    }

    fn refs(&self) -> Box<[TypePtr]> {
        let fields = self.fields.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();
        let impls = self
            .impls
            .iter()
            .flat_map(|im| im.refs())
            .collect::<Vec<_>>();
        vec![fields, impls].concat().into_boxed_slice()
    }
}

impl From<StructType> for Type {
    fn from(value: StructType) -> Self {
        Self::Struct(value)
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
