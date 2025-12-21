use std::rc::Rc;

use crate::Location;

///
/// ## Bytes
/// an immutable sequence of bytes, optimized
/// for reference sharing.
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn first(&self) -> Location {
        self.location(0)
    }

    #[inline]
    pub fn last(&self) -> Location {
        self.location(self.len() - 1)
    }

    #[inline]
    pub fn location(&self, index: usize) -> Location {
        Location::new(index, &self.0)
    }
}

impl From<Vec<u8>> for Bytes {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self(Rc::from(value))
    }
}

impl From<&[u8]> for Bytes {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(Rc::from(value))
    }
}

impl From<&str> for Bytes {
    #[inline]
    fn from(value: &str) -> Self {
        Self(Rc::from(value.as_bytes()))
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Bytes {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = String::from_utf8(self.0.to_vec())
            .expect("error while attempting to convert Bytes to utf8 String");
        write!(f, "{}", &value)
    }
}
