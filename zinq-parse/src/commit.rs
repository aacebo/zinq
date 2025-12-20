use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

static ID: AtomicU64 = AtomicU64::new(1);

///
/// ## Commit
/// used to save state
///
#[derive(Debug, Clone, Eq)]
pub struct Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    id: u64,
    value: T,
    time: Instant,
}

impl<T> Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    #[inline]
    pub fn id(&self) -> &u64 {
        &self.id
    }

    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn time(&self) -> &Instant {
        &self.time
    }
}

impl<T> From<T> for Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            value,
            time: std::time::Instant::now(),
        }
    }
}

impl<T> std::ops::Deref for Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::fmt::Display for Commit<T>
where
    T: std::fmt::Debug,
    T: std::fmt::Display,
    T: Clone,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Commit #{}\n", self.id)?;
        write!(f, "{}", &self.value)
    }
}

impl<T> PartialEq for Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
