mod type_entry;
mod type_registry;

pub use type_entry::*;
pub use type_registry::*;

#[macro_export]
macro_rules! import {
    () => {
        thread_local! {
            static REGISTRY: ::std::cell::RefCell<::macrox::registry::TypeRegistry> =
                ::std::cell::RefCell::new(::macrox::registry::TypeRegistry::new());
        }

        fn count() -> usize {
            REGISTRY.with_borrow(|registry| registry.count())
        }

        fn has(key: &str) -> bool {
            REGISTRY.with_borrow(|registry| registry.has(key))
        }

        fn get(key: &str) -> Option<::macrox::registry::TypeEntry> {
            REGISTRY.with_borrow(|registry| registry.get(key).cloned())
        }

        fn put(key: String, value: ::macrox::registry::TypeEntry) {
            REGISTRY.with_borrow_mut(|registry| registry.put(key, value))
        }
    };
}
