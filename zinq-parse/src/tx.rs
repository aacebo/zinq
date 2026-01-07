use std::{
    ops::Index,
    sync::atomic::{AtomicU64, Ordering},
};

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
#[derive(Debug, Default, Clone, Eq)]
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
    pub fn new(value: T) -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            commits: vec![value.into()],
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
        self.commits.len() < 2
    }

    /// ## first
    /// get the first commit value
    #[inline]
    pub fn first(&self) -> &T {
        self.commits
            .first()
            .expect("expected at least 1 commit in transaction")
            .value()
    }

    ///
    /// ## last
    /// the last commit value
    ///
    #[inline]
    pub fn last(&self) -> &T {
        self.commits
            .last()
            .expect("expected at least 1 commit in transaction")
            .value()
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
    /// ## append
    /// append the commits from one tx to another
    ///
    #[inline]
    pub fn append(&mut self, mut tx: Self) {
        self.commits.append(&mut tx.commits);
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
    /// ## squash
    /// create a new commit to reduce the transactions commit count to
    /// 2 (start -> end) when possible
    ///
    #[inline]
    pub fn squash(self) -> Self {
        if self.commits.len() > 2 {
            return self;
        }

        let first = self.commits.first().unwrap().clone();
        let last = self.commits.last().unwrap().clone();

        Self {
            id: self.id,
            commits: vec![first, last],
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
