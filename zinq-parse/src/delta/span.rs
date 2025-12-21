#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: super::Location,
    end: super::Location,
    steps: usize,
}

impl Span {
    ///
    /// ## start
    /// get the delta for the two spans
    /// start locations
    ///
    #[inline]
    pub fn start(&self) -> &super::Location {
        &self.start
    }

    ///
    /// ## end
    /// get the delta for the two spans
    /// end locations
    ///
    #[inline]
    pub fn end(&self) -> &super::Location {
        &self.end
    }

    ///
    /// ## steps
    /// get the euclidean distance between the two spans
    /// using their location deltas
    ///
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }
}

impl std::fmt::Display for Span {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span ({})\n", self.steps())?;
        write!(f, "Start: {}\n", self.start())?;
        write!(f, "End:   {}\n", self.end())
    }
}

impl super::Delta for crate::Span {
    type Output = Span;

    #[inline]
    fn delta(start: &Self, end: &Self) -> Self::Output {
        let start_diff = *end.start() - *start.start();
        let end_diff = *end.end() - *start.end();

        Self::Output {
            start: start_diff,
            end: end_diff,
            steps: start_diff.steps().pow(2) + end_diff.steps().pow(2),
        }
    }
}
