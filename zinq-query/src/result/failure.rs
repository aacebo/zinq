use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Failure {
    #[cfg_attr(feature = "serde", serde(rename = "$meta"))]
    pub meta: BTreeMap<String, zinq_reflect::Value>,

    #[cfg_attr(feature = "serde", serde(rename = "error"))]
    pub error: zinq_schema::error::Error,
}

impl Failure {
    pub fn new(error: zinq_schema::error::Error) -> Self {
        return Self {
            meta: BTreeMap::new(),
            error,
        };
    }

    pub fn into_inner(&self) -> zinq_schema::error::Error {
        return self.error.clone();
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.error);
    }
}

impl From<zinq_schema::error::Error> for Failure {
    fn from(value: zinq_schema::error::Error) -> Self {
        return Self::new(value);
    }
}

impl From<&zinq_schema::error::Error> for Failure {
    fn from(value: &zinq_schema::error::Error) -> Self {
        return Self::new(value.clone());
    }
}

impl Into<zinq_schema::error::Error> for Failure {
    fn into(self) -> zinq_schema::error::Error {
        return self.into_inner();
    }
}
