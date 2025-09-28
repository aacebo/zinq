use std::collections::BTreeMap;

use crate::TypeOf;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct MetaData(BTreeMap<String, crate::Value>);

impl MetaData {
    pub fn new() -> Self {
        return Self(BTreeMap::new());
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, crate::Value> {
        return self.0.iter();
    }

    pub fn has(&self, key: &str) -> bool {
        return self.0.contains_key(key);
    }

    pub fn set(&mut self, key: &str, value: &crate::Value) {
        self.0.insert(key.to_string(), value.clone());
    }

    pub fn get(&self, key: &str) -> Option<&crate::Value> {
        return self.0.get(key);
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut crate::Value> {
        return self.0.get_mut(key);
    }

    pub fn merge(mut self, other: &Self) -> Self {
        for (key, value) in &other.0 {
            self.set(&key, &value);
        }

        return self;
    }
}

impl<const N: usize> From<[(String, crate::Value); N]> for MetaData {
    fn from(value: [(String, crate::Value); N]) -> Self {
        return Self(BTreeMap::from(value));
    }
}

impl std::fmt::Display for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.0 {
            write!(f, "\n\t{}: {}", key, value)?;
        }

        if self.0.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}

impl crate::Reflect for MetaData {
    fn reflect(self) -> crate::Value {
        let mut map = crate::Map::new(&crate::type_of!(BTreeMap<String, crate::Value>).to_map());

        for (key, value) in &self.0 {
            map.set(&key.reflect(), value);
        }

        return crate::Value::Map(map);
    }
}
