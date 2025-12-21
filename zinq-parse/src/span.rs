use std::ops::Index;

use zinq_error::{AnyError, Error, Result, TextError};

use crate::{
    Bytes, FileMetaData, Location, ParseError,
    delta::{self, Delta},
};

///
/// ## Span
/// an immutable slice of bytes bound by a start and
/// end location.
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub(crate) start: Location,
    pub(crate) end: Location,
    pub(crate) file: Option<FileMetaData>,
    pub(crate) bytes: Bytes,
}

impl Span {
    #[inline]
    pub fn from_bytes(src: &[u8]) -> Result<Self> {
        let bytes = Bytes::from(src);

        Ok(Self {
            start: bytes.first(),
            end: bytes.last(),
            file: None,
            bytes,
        })
    }

    #[inline]
    pub fn from_str(src: &str) -> Result<Self> {
        let bytes = Bytes::from(src);

        Ok(Self {
            start: bytes.first(),
            end: bytes.last(),
            file: None,
            bytes,
        })
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
    pub fn end(&self) -> &Location {
        &self.end
    }

    #[inline]
    pub fn first(&self) -> &u8 {
        self.bytes.index(self.start.index)
    }

    #[inline]
    pub fn last(&self) -> &u8 {
        self.bytes.index(self.end.index)
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
    pub fn matches(&self, bytes: &[u8]) -> bool {
        self.bytes() == bytes
    }

    #[inline]
    pub fn slice(&self, start: Location, end: Location) -> Self {
        Self {
            start,
            end,
            file: self.file.clone(),
            bytes: self.bytes.clone(),
        }
    }

    #[inline]
    pub fn error(&self, message: &str) -> Error {
        AnyError::new(ParseError::new(self.clone(), TextError::from(message))).into()
    }
}

impl std::fmt::Display for Span {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = String::from_utf8(self.bytes().to_vec()).expect("utf8 source reading failed");
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
