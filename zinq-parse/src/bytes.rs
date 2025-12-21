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
        self.at(0)
    }

    #[inline]
    pub fn last(&self) -> Location {
        self.at(self.len() - 1)
    }

    #[inline]
    pub fn at(&self, index: usize) -> Location {
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

impl<const N: usize> From<&[u8; N]> for Bytes {
    #[inline]
    fn from(value: &[u8; N]) -> Self {
        Self(Rc::from(value.as_ref()))
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
        let value = str::from_utf8(&self.0)
            .expect("error while attempting to convert Bytes to utf8 String");
        write!(f, "{}", &value)
    }
}

#[cfg(test)]
mod test {
    use crate::{Bytes, Location};

    #[test]
    fn should_create_bytes() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");

        debug_assert_eq!(bytes.len(), 23);
        debug_assert_eq!(bytes.first(), Location::new(0, &bytes));
        debug_assert_eq!(bytes.last(), Location::new(bytes.len() - 1, &bytes));
    }

    #[test]
    fn should_create_locations() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let mut location = bytes.at(3);

        debug_assert_eq!(location.index(), 3);
        debug_assert_eq!(location.line(), 1);
        debug_assert_eq!(location.column(), 0);
        debug_assert_eq!(bytes[location.index()], b'm');

        location = bytes.at(15);

        debug_assert_eq!(location.index(), 15);
        debug_assert_eq!(location.line(), 6);
        debug_assert_eq!(location.column(), 1);
        debug_assert_eq!(bytes[location.index()], b's');
    }
}
