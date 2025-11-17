use zinq_error::ToError;
use zinq_error_macros::*;

#[derive(Error, Debug)]
#[error(name = "SomeErrorName")]
pub enum CustomError {
    UnAuthorized,
}

#[test]
pub fn should_create_error() {
    let err = CustomError::UnAuthorized.to_error();
    debug_assert_eq!(err.to_string(), "UnAuthorized");
    debug_assert_eq!(err.message(), Some("UnAuthorized"));
    debug_assert_eq!(err.name(), Some("SomeErrorName"));
}
