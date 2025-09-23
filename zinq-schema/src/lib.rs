mod any;
pub mod error;
mod string;

#[cfg(feature = "macros")]
pub use zinq_schema_macros::*;

pub use any::*;
pub use string::*;

/// ## Schema
///
/// type definition/metadata used to
/// perform various zinq supported tasks
pub trait Schema {
    /// gets the unique schema name
    fn name(&self) -> &'static str;
}
