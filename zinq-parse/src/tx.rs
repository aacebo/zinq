use crate::Commit;

///
/// ## Tx
/// tracks the incremental changes being
/// made by a parser for a unit of work
///
#[derive(Debug, Clone)]
pub struct Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
    commits: Vec<Commit<T>>,
}

impl<T> Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
    ///
    /// ## new
    /// create a new empty transaction
    ///
    #[inline]
    pub fn new() -> Self {
        Self { commits: vec![] }
    }

    ///
    /// ## value
    /// the most recent commit
    ///
    #[inline]
    pub fn value(&self) -> Option<&Commit<T>> {
        self.commits.last()
    }

    ///
    /// ## next
    /// set the current `Span` and
    /// add a delta to the trace
    ///
    #[inline]
    pub fn next(&mut self, value: T) {
        self.commits.push(value.into());
    }

    ///
    /// ## commits
    /// an ordered slice of the commits that
    /// make up this trace
    ///
    #[inline]
    pub fn commits(&self) -> &[Commit<T>] {
        &self.commits
    }
}

impl<T> From<T> for Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            commits: vec![value.into()],
        }
    }
}

impl<T> std::fmt::Display for Tx<T>
where
    T: std::fmt::Debug,
    T: std::fmt::Display,
    T: Clone,
    T: Eq,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tx ({}):\n", self.commits.len())?;

        for commit in &self.commits {
            write!(f, "{}", commit)?;
        }

        Ok(())
    }
}
