use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Map(BTreeMap<String, String>);

impl Map {
    pub fn new() -> Self {
        return Self(BTreeMap::new());
    }
}

impl From<BTreeMap<String, String>> for Map {
    fn from(value: BTreeMap<String, String>) -> Self {
        return Self(value);
    }
}
