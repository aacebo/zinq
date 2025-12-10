use std::rc::Rc;

use crate::{Config, Item, Key};

#[derive(Clone)]
pub struct Section {
    key: Key,
    value: Rc<dyn Config>,
}

impl Section {
    pub fn new<T: Config + 'static>(key: Key, value: T) -> Self {
        return Self {
            key,
            value: Rc::new(value),
        };
    }

    pub fn key(&self) -> &Key {
        return &self.key;
    }
}

impl std::ops::Deref for Section {
    type Target = dyn Config + 'static;

    fn deref(&self) -> &Self::Target {
        return self.value.as_ref();
    }
}

impl Config for Section {
    fn item(&self, key: &Key) -> Option<Item> {
        self.value.item(key)
    }

    fn children(&self) -> Vec<Item> {
        self.value.children()
    }
}
