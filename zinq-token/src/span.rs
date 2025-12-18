use std::rc::Rc;

use crate::{Location, source::File};

///
/// ## Span
/// a range of bytes in a source file
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub(crate) start: Location,
    pub(crate) end: Location,
    pub(crate) file: Rc<File>,
}

impl Span {
    #[inline]
    pub fn start(&self) -> Location {
        self.start
    }

    #[inline]
    pub fn end(&self) -> Location {
        self.end
    }

    #[inline]
    pub fn file(&self) -> &File {
        self.file.as_ref()
    }

    #[inline]
    pub fn sof(&self) -> bool {
        self.end.index() == 0
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.end.index() == self.file.bytes().len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end.index() - self.start.index()
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.file.bytes()[self.start.index()..self.end.index()]
    }

    #[inline]
    pub fn range(&self, start: Location, end: Location) -> Self {
        Self {
            start,
            end,
            file: self.file.clone(),
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = String::from_utf8(self.bytes().to_vec()).expect("utf8 source reading failed");
        write!(f, "{}", &value)
    }
}
