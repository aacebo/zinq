use std::collections::HashMap;

use crate::registry::TypeEntry;

#[derive(Clone)]
pub struct TypeRegistry(HashMap<String, TypeEntry>);

impl TypeRegistry {
    pub fn new() -> Self {
        return Self(HashMap::new());
    }

    pub fn count(&self) -> usize {
        return self.0.len();
    }

    pub fn has(&self, key: &str) -> bool {
        return self.0.contains_key(key);
    }

    pub fn get(&self, key: &str) -> Option<&TypeEntry> {
        return self.0.get(key);
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut TypeEntry> {
        return self.0.get_mut(key);
    }

    pub fn put(&mut self, key: String, value: TypeEntry) {
        self.0.insert(key, value);
    }
}
