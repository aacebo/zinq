use zinq_error::ToError;
use zinq_error_macros::*;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error(message = "my test message...")]
    UnAuthorized,
}

#[test]
pub fn should_create_error() {
    let err = CustomError::UnAuthorized.to_error();
    debug_assert_eq!(err.to_string(), "UnAuthorized: my test message...");
    debug_assert_eq!(err.message(), Some("my test message..."));
    debug_assert_eq!(err.name(), Some("UnAuthorized"));
    debug_assert_eq!(err.code(), None);
}
