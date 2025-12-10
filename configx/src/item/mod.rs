mod section;
mod value;

pub use section::*;
pub use value::*;

use crate::{Config, Key};

#[derive(Clone)]
pub enum Item {
    Section(Section),
    Value(Value),
}

impl Item {
    pub fn key(&self) -> &Key {
        return match self {
            Self::Section(v) => v.key(),
            Self::Value(v) => v.key(),
        };
    }
}

impl Config for Item {
    fn item(&self, key: &Key) -> Option<Item> {
        match self {
            Self::Section(v) => v.item(key),
            _ => None,
        }
    }
}

impl AsRef<Item> for Item {
    fn as_ref(&self) -> &Item {
        self
    }
}
