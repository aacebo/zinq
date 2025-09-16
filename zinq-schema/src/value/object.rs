use std::{collections::BTreeMap, fmt};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Object(BTreeMap<String, super::Value>);

impl Object {
    pub fn new() -> Self {
        return Self(BTreeMap::new());
    }

    pub fn merge(a: &Self, b: &Self) -> Self {
        let mut object = Self::new();

        for (key, value) in a {
            object.set(key, value.clone());
        }

        for (key, value) in b {
            object.set(key, value.clone());
        }

        return object;
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn get(&self, key: &str) -> Option<&super::Value> {
        return self.0.get(key);
    }

    pub fn set(&mut self, key: &str, value: super::Value) {
        self.0.insert(key.to_string(), value);
    }

    pub fn del(&mut self, key: &str) {
        self.0.remove(key);
    }
}

impl From<BTreeMap<String, super::Value>> for Object {
    fn from(value: BTreeMap<String, super::Value>) -> Self {
        return Self(value);
    }
}

impl Into<BTreeMap<String, super::Value>> for Object {
    fn into(self) -> BTreeMap<String, super::Value> {
        return self.0;
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.0.iter().enumerate() {
            write!(f, "\"{}\": {}", key, value)?;

            if i < self.0.len() - 1 {
                write!(f, ",")?;
            }
        }

        return write!(f, "}}");
    }
}

impl IntoIterator for Object {
    type IntoIter = std::collections::btree_map::IntoIter<String, super::Value>;
    type Item = (String, super::Value);

    fn into_iter(self) -> Self::IntoIter {
        return self.0.into_iter();
    }
}

impl<'a> IntoIterator for &'a Object {
    type IntoIter = std::collections::btree_map::Iter<'a, String, super::Value>;
    type Item = (&'a String, &'a super::Value);

    fn into_iter(self) -> Self::IntoIter {
        return self.0.iter();
    }
}
