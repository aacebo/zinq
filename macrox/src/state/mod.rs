mod id;

pub use id::*;

///
/// ## State
/// some state that can be forked and merged
///
pub trait State {
    ///
    /// ## fork
    /// create a copy of the state for editting
    ///
    fn fork(&self) -> Self;

    ///
    /// ## merge
    /// merge another state into this state
    ///
    fn merge(&mut self, other: Self) -> Result<(), crate::Error>;
}

///
/// ## Record
/// a uniquely identifiable item to be managed by
/// a `State` collection
///
pub trait Record {
    ///
    /// ## id
    /// the unique identifier of the record
    ///
    fn id(&self) -> &Id;
}
