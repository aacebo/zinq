use std::{collections::HashMap, hash::Hash};

use zinq_parse::{Span, Spanned};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Arena<K: Eq + Hash, V: Spanned> {
    items: Vec<V>,
    source: HashMap<K, Span>,
}

impl<K: Eq + Hash, V: Spanned> Arena<K, V> {
    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, key: K, item: V) {
        self.source.insert(key, item.span());
        self.items.push(item);
    }

    pub fn get(&self, key: &K) -> Option<&Span> {
        self.source.get(key)
    }
}

impl<K: Eq + Hash, V: Spanned> std::ops::Index<usize> for Arena<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.items.index(index)
    }
}
