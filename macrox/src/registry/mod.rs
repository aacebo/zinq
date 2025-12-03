mod type_entry;
mod type_registry;

use std::cell::RefCell;

pub use type_entry::*;
pub use type_registry::*;

thread_local! {
    static REGISTRY: RefCell<TypeRegistry> =
        RefCell::new(TypeRegistry::new());
}

pub fn count() -> usize {
    REGISTRY.with_borrow(|registry| registry.count())
}

pub fn has(key: &str) -> bool {
    REGISTRY.with_borrow(|registry| registry.has(key))
}

pub fn get(key: &str) -> Option<TypeEntry> {
    REGISTRY.with_borrow(|registry| registry.get(key).cloned())
}

pub fn put(key: String, value: TypeEntry) {
    REGISTRY.with_borrow_mut(|registry| registry.put(key, value))
}
