///
/// ## delta::Location
/// describes the delta (diff) between a start and end `Location`.
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    steps: usize,
    lines: usize,
    columns: isize,
}

impl Location {
    ///
    /// ## steps
    /// the index delta
    ///
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }

    ///
    /// ## lines
    /// the line delta
    ///
    #[inline]
    pub fn lines(&self) -> usize {
        self.lines
    }

    ///
    /// ## columns
    /// the column delta
    ///
    #[inline]
    pub fn columns(&self) -> isize {
        self.columns
    }
}

impl std::fmt::Display for Location {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lns {}, Cols {}, Steps {}",
            self.lines(),
            self.columns(),
            self.steps()
        )
    }
}

impl super::Delta for crate::Location {
    type Output = Location;

    #[inline]
    fn delta(start: &Self, end: &Self) -> Self::Output {
        Self::Output {
            steps: end.index() - start.index(),
            lines: end.line() - start.line(),
            columns: end.column().cast_signed() - start.column().cast_signed(),
        }
    }
}
