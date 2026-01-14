use std::{collections::HashMap, hash::Hash};

use zinq_parse::Span;

#[derive(Clone, PartialEq, Eq)]
pub struct Arena<K: Eq + Hash, V> {
    items: HashMap<K, (V, Span)>,
}

impl<K: Eq + Hash, V> Arena<K, V> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn exists(&self, key: &K) -> bool {
        self.items.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Some(&self.items.get(key)?.0)
    }

    pub fn get_span(&self, key: &K) -> Option<&Span> {
        Some(&self.items.get(key)?.1)
    }

    pub fn require(&self, key: &K) -> &V {
        self.get(key).expect("unexpected key")
    }

    pub fn add(&mut self, key: K, item: V, span: Span) {
        self.items.insert(key, (item, span));
    }
}

impl<K: Eq + Hash, V> std::ops::Index<&K> for Arena<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        &self.items.index(index).0
    }
}
