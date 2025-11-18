use zinq_error::ToError;
use zinq_error_macros::*;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error(name = "hello", message = "world")]
    UnAuthorized,
}

#[test]
pub fn should_create_error() {
    let err = CustomError::UnAuthorized.to_error();
    debug_assert_eq!(err.to_string(), "hello: world");
    debug_assert_eq!(err.message(), Some("world"));
    debug_assert_eq!(err.name(), Some("hello"));
}
