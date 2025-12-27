use std::ops::Index;

use zinq_error::Result;

use crate::{
    Bytes, Cursor, FileMetaData, Location,
    delta::{self, Delta},
};

///
/// ## Span
/// an immutable slice of bytes bound by a start and
/// end location.
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: Location,
    end: Location,
    file: Option<FileMetaData>,
    bytes: Bytes,
}

impl Span {
    #[inline]
    pub fn from_bytes(src: &[u8]) -> Self {
        let bytes = Bytes::from(src);

        Self {
            start: bytes.first(),
            end: bytes.last(),
            file: None,
            bytes,
        }
    }

    #[inline]
    pub fn from_str(src: &str) -> Self {
        let bytes = Bytes::from(src);

        Self {
            start: bytes.first(),
            end: bytes.last(),
            file: None,
            bytes,
        }
    }

    #[inline]
    pub fn from_file(path: &str) -> Result<Self> {
        let file = FileMetaData::try_from(path)?;
        let bytes = file.read()?;

        Ok(Self {
            start: bytes.first(),
            end: bytes.last(),
            file: Some(file),
            bytes,
        })
    }
}

impl Span {
    #[inline]
    pub fn start(&self) -> &Location {
        &self.start
    }

    #[inline]
    pub fn start_mut(&mut self) -> &mut Location {
        &mut self.start
    }

    #[inline]
    pub fn end(&self) -> &Location {
        &self.end
    }

    #[inline]
    pub fn end_mut(&mut self) -> &mut Location {
        &mut self.end
    }

    #[inline]
    pub fn first(&self) -> &u8 {
        self.bytes.index(self.start.index())
    }

    #[inline]
    pub fn last(&self) -> &u8 {
        if self.sof() {
            return &0;
        }

        self.bytes.index(self.end.index() - 1)
    }

    #[inline]
    pub fn file(&self) -> Option<&FileMetaData> {
        self.file.as_ref()
    }

    #[inline]
    pub fn sof(&self) -> bool {
        self.end.index() == 0
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.end.index() == self.bytes.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end.index() - self.start.index()
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[self.start.index()..self.end.index()]
    }

    #[inline]
    pub fn src(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn slice(&self, from: usize, to: usize) -> Self {
        let start = Location::new(from, &self.bytes);
        let end = Location::new(to, &self.bytes);

        Self {
            start,
            end,
            file: self.file.clone(),
            bytes: self.bytes.clone(),
        }
    }

    #[inline]
    pub fn cursor(&self) -> Cursor {
        let mut span = self.clone();
        span.end = span.start;
        Cursor::from(span)
    }
}

impl std::fmt::Display for Span {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = str::from_utf8(self.bytes()).expect("utf8 source reading failed");
        write!(f, "{}", &value)
    }
}

impl std::ops::Sub for Span {
    type Output = delta::Span;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::delta(&self, &rhs)
    }
}

impl PartialEq<&[u8]> for Span {
    fn eq(&self, other: &&[u8]) -> bool {
        self.bytes() == *other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for Span {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.bytes() == *other
    }
}

#[cfg(test)]
mod test {
    use crate::{Bytes, Span};

    #[test]
    fn should_create_span() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);

        debug_assert_eq!(span.bytes(), bytes.as_ref());
        debug_assert!(span.eof());
        debug_assert_eq!(span.len(), bytes.len());
    }

    #[test]
    fn should_create_sub_span() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes).slice(7, 11);

        debug_assert_eq!(span.bytes(), b"name");
        debug_assert!(!span.eof());
        debug_assert_eq!(span.len(), 4);
    }

    #[test]
    fn should_create_delta() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);
        let a = span.slice(3, 5);
        let b = span.slice(4, 8);

        debug_assert_eq!(a.bytes(), b"my");
        debug_assert_eq!(b.bytes(), b"y\n\nn");

        let delta = b - a;

        debug_assert_eq!(delta.start().steps(), 1);
        debug_assert_eq!(delta.start().lines(), 0);
        debug_assert_eq!(delta.start().columns(), 1);

        debug_assert_eq!(delta.end().steps(), 3);
        debug_assert_eq!(delta.end().lines(), 2);
        debug_assert_eq!(delta.end().columns(), -1);
    }
}
