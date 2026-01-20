use std::collections::HashMap;

use zinq_reflect::ty::{Type, TypeId, ZinqType};

use crate::types::TypeEntry;

#[derive(Debug)]
pub struct TypeRegistry {
    items: HashMap<TypeId, TypeEntry>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn exists(&self, ty: Type) -> bool {
        self.items.contains_key(&ty.id())
    }

    pub fn get(&self, id: &TypeId) -> Option<&Type> {
        let entry = match self.items.get(id) {
            None => return None,
            Some(v) => v,
        };

        Some(&entry.ty)
    }

    pub fn require(&self, id: &TypeId) -> &Type {
        self.get(id).unwrap()
    }

    pub fn add(&mut self, ty: Type) -> &mut Self {
        self.items.insert(ty.id(), TypeEntry::from(ty));
        self
    }
}
