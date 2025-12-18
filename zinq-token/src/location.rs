///
/// ## Location
/// a 0 indexed coordinate in a file
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
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ln {}, Col {}", self.line + 1, self.column + 1)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line
            .cmp(&other.line)
            .then(self.column.cmp(&other.column))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
