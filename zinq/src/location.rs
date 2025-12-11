#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    row: usize,
    col: usize,
}

impl Location {
    #[inline]
    pub fn line(&self) -> usize {
        return self.row + 1;
    }

    #[inline]
    pub fn column(&self) -> usize {
        return self.col + 1;
    }
}
