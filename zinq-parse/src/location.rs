use std::ops::Index;

use crate::delta::{self, Delta};

///
/// ## Location
/// a 0 indexed coordinate in a set of bytes
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    index: usize,
    line: usize,
    column: usize,
}

impl Location {
    ///
    /// ## new
    /// calculate a new location at a given `index`
    /// of some `bytes`
    ///
    #[inline]
    pub fn new(index: usize, bytes: &[u8]) -> Self {
        let mut location = Self::default();
        location.seek(index, bytes);
        return location;
    }

    ///
    /// ## index
    /// the raw index
    ///
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    ///
    /// ## line
    /// the line index
    ///
    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    ///
    /// ## column
    /// the column index
    ///
    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

    ///
    /// ## seek
    /// change the location to the given `index` and
    /// recalculate the `line` and `column`
    ///
    #[inline]
    pub fn seek(&mut self, index: usize, bytes: &[u8]) -> bool {
        let mut i = 0;
        let mut line = 0;
        let mut column = 0;

        while i < index {
            i += 1;
            column += 1;

            let byte = match bytes.get(i) {
                None => return false,
                Some(v) => v,
            };

            if byte == &b'\n' {
                line += 1;
                column = 0;
            }
        }

        self.index = i;
        self.line = line;
        self.column = column;
        true
    }

    ///
    /// ## next
    /// move the location forward by 1
    ///
    #[inline]
    pub fn next(&mut self, bytes: &[u8]) -> bool {
        if self.index + 1 > bytes.len() - 1 {
            return false;
        }

        self.index += 1;
        self.column += 1;

        if bytes.index(self.index) == &b'\n' {
            self.line += 1;
            self.column = 0;
        }

        true
    }

    ///
    /// ## back
    /// move the location backward by 1
    ///
    #[inline]
    pub fn back(&mut self, bytes: &[u8]) -> bool {
        if self.index == 0 {
            return false;
        }

        let mut column = self.column - 1;
        let mut line = self.line;

        if bytes.index(self.index) == &b'\n' {
            let mut count = 0;

            for i in (0..self.index).rev() {
                if bytes.index(i) == &b'\n' {
                    break;
                }

                count += 1;
            }

            line -= 1;
            column = count;
        }

        self.index -= 1;
        self.column = column;
        self.line = line;
        true
    }
}

impl std::fmt::Display for Location {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ln {}, Col {}", self.line + 1, self.column + 1)
    }
}

impl Ord for Location {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line
            .cmp(&other.line)
            .then(self.column.cmp(&other.column))
    }
}

impl PartialOrd for Location {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Sub for Location {
    type Output = delta::Location;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::delta(&self, &rhs)
    }
}
