mod commit;
mod location;
mod span;

pub use commit::*;
pub use location::*;
pub use span::*;

pub trait Delta {
    type Output;

    ///
    /// ## between
    /// calculate the delta between two commits
    ///
    fn delta(start: &Self, end: &Self) -> Self::Output;
}
