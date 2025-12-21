use std::{
    ops::Index,
    sync::atomic::{AtomicU64, Ordering},
};

use zinq_error::Result;

use crate::{
    Commit,
    delta::{self, Delta},
};

static ID: AtomicU64 = AtomicU64::new(1);

///
/// ## Tx
/// tracks the incremental changes being
/// made by a parser for a unit of work
///
#[derive(Debug, Clone, Eq)]
pub struct Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    id: u64,
    commits: Vec<Commit<T>>,
}

impl<T> Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    ///
    /// ## new
    /// create a new empty transaction
    ///
    #[inline]
    pub fn new() -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            commits: vec![],
        }
    }

    ///
    /// ## id
    /// the transaction id
    ///
    #[inline]
    pub fn id(&self) -> &u64 {
        &self.id
    }

    ///
    /// ## empty
    /// check if the transaction is empty
    ///
    #[inline]
    pub fn empty(&self) -> bool {
        self.commits.is_empty()
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
    /// ## commits
    /// an ordered slice of the commits that
    /// make up this trace
    ///
    #[inline]
    pub fn commits(&self) -> &[Commit<T>] {
        &self.commits
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
    /// ## revert
    /// revert back to the state of the commit
    /// before our current commit
    ///
    #[inline]
    pub fn revert(&mut self) -> Option<Commit<T>> {
        self.commits.pop()
    }

    ///
    /// ## run
    /// runs your handler and, if successful, adds a new commit
    /// to the transaction
    ///
    #[inline]
    pub fn run<Handler: FnOnce() -> Result<T>>(&mut self, handler: Handler) -> Result<&Commit<T>> {
        match handler() {
            Err(err) => Err(err),
            Ok(v) => {
                self.commits.push(Commit::from(v));
                Ok(self
                    .commits
                    .last()
                    .expect("expected at least one commit to be in transaction"))
            }
        }
    }
}

impl<T> Tx<T>
where
    T: std::fmt::Debug,
    T: delta::Delta,
    T: Clone,
{
    ///
    /// ## delta
    /// get the delta of the first and last commit
    ///
    #[inline]
    pub fn delta(&self) -> delta::Commit<T::Output> {
        let first = self
            .commits
            .first()
            .expect("expected at least one commit in transaction");
        let last = self.commits.last().unwrap();

        Commit::<T>::delta(first, last)
    }

    ///
    /// ## delta_slice
    /// get the delta of a sub slice by giving the indices
    /// of the upper and lower bounded commits
    ///
    #[inline]
    pub fn delta_slice(&self, start: usize, end: usize) -> delta::Commit<T::Output> {
        let first = self.commits.index(start);
        let last = self.commits.index(end);
        Commit::<T>::delta(first, last)
    }
}

impl<T> From<T> for Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            commits: vec![value.into()],
        }
    }
}

impl<T> std::fmt::Display for Tx<T>
where
    T: std::fmt::Debug,
    T: std::fmt::Display,
    T: Clone,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tx #{} ({}):\n", self.id, self.commits.len())?;

        for commit in &self.commits {
            write!(f, "{}", commit)?;
        }

        Ok(())
    }
}

impl<T> PartialEq for Tx<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
