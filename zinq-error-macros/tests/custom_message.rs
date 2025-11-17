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
    debug_assert_eq!(err.message(), Some("UnAuthorized: my test message..."));
    debug_assert_eq!(err.name(), Some("CustomError"));
    debug_assert_eq!(err.code(), None);
}
