use crate::delta::Delta;

///
/// ## Commit
/// a delta of two commits
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commit<T> {
    delta: T,
    elapse: std::time::Duration,
}

impl<T: Delta> Commit<T> {
    ///
    /// ## elapse
    /// get the elapse time between the two commits
    ///
    #[inline]
    pub fn elapse(&self) -> &std::time::Duration {
        &self.elapse
    }
}

impl<T: Delta> std::fmt::Display for Commit<T>
where
    T: std::fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.delta)
    }
}

impl<T: Delta> Delta for crate::Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
    type Output = super::Commit<T::Output>;

    #[inline]
    fn delta(start: &Self, end: &Self) -> Self::Output {
        Self::Output {
            delta: T::delta(start.value(), end.value()),
            elapse: end.time().duration_since(*start.time()),
        }
    }
}
