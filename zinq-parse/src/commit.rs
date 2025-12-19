use std::time::Instant;

///
/// ## Commit
/// used to save state
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
    value: T,
    time: Instant,
}

impl<T> Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
{
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
    T: Eq,
{
    #[inline]
    fn from(value: T) -> Self {
        Self {
            value,
            time: std::time::Instant::now(),
        }
    }
}

impl<T> std::ops::Deref for Commit<T>
where
    T: std::fmt::Debug,
    T: Clone,
    T: Eq,
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
    T: Eq,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}
