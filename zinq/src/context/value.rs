use std::str::FromStr;

use crate::error::{DynError, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(String);

impl Value {
    pub fn get<T: FromStr>(&self) -> Result<T>
    where
        T::Err: std::error::Error + 'static,
    {
        match T::from_str(&self.0) {
            Err(err) => Err(DynError::new(err).into()),
            Ok(v) => Ok(v),
        }
    }

    pub fn get_required<T: FromStr>(&self) -> T
    where
        T::Err: std::error::Error + 'static,
    {
        match T::from_str(&self.0) {
            Err(err) => panic!("{}", err),
            Ok(v) => v,
        }
    }

    pub fn set<T: ToString>(&mut self, value: &T) {
        self.0 = value.to_string();
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl<T: ToString> From<T> for Value {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

impl std::ops::Deref for Value {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}
