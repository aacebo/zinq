use std::collections::HashMap;

use zinq_error::{Error, Result};
use zinq_reflect::ty::{
    BoolType, FloatType, IntType, PtrType, StringType, Type, TypeId, UIntType, ZinqType,
};

use crate::types::TypeEntry;

#[derive(Debug)]
pub struct TypeRegistry {
    items: HashMap<TypeId, TypeEntry>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut value = Self {
            items: HashMap::new(),
        };

        value.add(BoolType.into()).unwrap();
        value.add(FloatType::F32.into()).unwrap();
        value.add(FloatType::F64.into()).unwrap();
        value.add(IntType::I8.into()).unwrap();
        value.add(IntType::I16.into()).unwrap();
        value.add(IntType::I32.into()).unwrap();
        value.add(IntType::I64.into()).unwrap();
        value.add(UIntType::U8.into()).unwrap();
        value.add(UIntType::U16.into()).unwrap();
        value.add(UIntType::U32.into()).unwrap();
        value.add(UIntType::U64.into()).unwrap();
        value.add(StringType.into()).unwrap();
        value
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn exists(&self, id: &TypeId) -> bool {
        self.items.contains_key(id)
    }

    pub fn get(&self, id: &TypeId) -> Option<&Type> {
        let entry = match self.items.get(id) {
            None => return None,
            Some(v) => v,
        };

        Some(&entry.ty)
    }

    pub fn get_or_err(&self, id: &TypeId) -> Result<&Type> {
        match self.items.get(id) {
            None => {
                return Err(Error::from_str(&format!("type {} not found", &id))
                    .build()
                    .into());
            }
            Some(v) => Ok(&v.ty),
        }
    }

    pub fn require(&self, id: &TypeId) -> &Type {
        self.get(id).unwrap()
    }

    pub fn add(&mut self, ty: Type) -> Result<&mut Self> {
        if self.exists(&ty.id()) {
            return Ok(self);
        }

        for ptr in ty.refs() {
            let edge = match self.items.get_mut(&ptr.id) {
                None => {
                    return Err(Error::from_str(&format!("type {} not found", &ptr))
                        .build()
                        .into());
                }
                Some(v) => v,
            };

            edge.inc_refs();
        }

        let ptr_ty: Type = Type::from(PtrType::from(ty.clone()));

        self.items.insert(ty.id(), TypeEntry::from(ty));
        self.items.insert(ptr_ty.id(), TypeEntry::from(ptr_ty));
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_reflect::{
        Field, TypePath, Variant,
        ty::{BoolType, EnumType, StructType, ZinqType},
    };

    use crate::types::TypeRegistry;

    #[test]
    fn should_have_refs() -> Result<()> {
        let mut registry = TypeRegistry::new();
        let a = StructType {
            path: TypePath::from("main::A"),
            fields: vec![Field {
                name: "empty".to_string(),
                ty: BoolType.ptr(),
            }],
            impls: vec![],
        };

        registry.add(a.clone().into()).unwrap();
        registry
            .add(
                EnumType {
                    path: TypePath::from("main::B"),
                    variants: vec![Variant {
                        path: TypePath::from("Main"),
                        fields: vec![Field {
                            name: "a".to_string(),
                            ty: a.ptr(),
                        }],
                    }],
                }
                .into(),
            )
            .unwrap();

        debug_assert_eq!(registry.len(), 28, "{:#?}", &registry);
        Ok(())
    }
}
