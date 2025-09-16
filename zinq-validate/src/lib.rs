use zinq_schema::Schema;

#[cfg(feature = "macros")]
pub use zinq_validate_macros::*;

/// ## Validate
///
/// enables validation for a zinq `Schema`
pub trait Validate: Schema {
    fn validate();
}
