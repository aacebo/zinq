use std::str::FromStr;

use zinq_error::ZinqError;

use crate::{Path, error::PathError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePath {
    pub module: Path,
    pub ident: String,
}

impl FromStr for TypePath {
    type Err = ZinqError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split("::").collect::<Vec<_>>();

        if let Some(last) = parts.pop() {
            return Ok(Self {
                module: Path::from(parts.as_slice()),
                ident: last.to_string(),
            });
        }

        Err(PathError::new(input).into())
    }
}

impl From<&str> for TypePath {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for TypePath {
    fn from(input: String) -> Self {
        Self::from_str(&input).expect("failed to parse type path")
    }
}

impl std::fmt::Display for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", &self.module, &self.ident)
    }
}
