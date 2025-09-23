use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Success<T> {
    #[cfg_attr(feature = "serde", serde(rename = "$meta"))]
    pub meta: BTreeMap<String, zinq_reflect::Value>,

    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub data: T,
}

impl<T> Success<T> {
    pub fn new(data: T) -> Self {
        return Self {
            meta: BTreeMap::new(),
            data,
        };
    }

    pub fn into_inner(self) -> T {
        return self.data;
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Success<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.data);
    }
}

impl<T> From<T> for Success<T> {
    fn from(value: T) -> Self {
        return Self::new(value);
    }
}
