#[cfg(feature = "macros")]
pub use zinq_validate_macros::*;

/// ## Validate
///
/// enables validation for a zinq `Schema`
pub trait Validate {
    fn validate();
}
