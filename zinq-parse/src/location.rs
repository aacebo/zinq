use std::ops::Index;

use crate::delta::{self, Delta};

///
/// ## Location
/// a 0 indexed coordinate in a set of bytes
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub(crate) index: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl Location {
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

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
