use std::str::FromStr;

use zinq_error::ZinqError;

use crate::error::PathError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    items: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl FromStr for Path {
    type Err = ZinqError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut items = vec![];

        for item in input.split("::") {
            items.push(item.to_string());
        }

        if items.is_empty() {
            return Err(PathError::new(input).into());
        }

        Ok(Self { items })
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for Path {
    fn from(input: String) -> Self {
        let mut items = vec![];

        for item in input.split("::") {
            items.push(item.to_string());
        }

        Self { items }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            write!(f, "{}", item)?;

            if i < self.items.len() - 1 {
                write!(f, "::")?;
            }
        }

        Ok(())
    }
}

impl std::ops::Deref for Path {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
