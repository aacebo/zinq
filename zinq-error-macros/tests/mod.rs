use zinq_error::{Error, ToError};

#[derive(Error, Debug)]
pub enum CustomError {
    BadRequest(String),
    UnAuthorized,
    #[error("some custom message")]
    Other,
}

#[test]
pub fn should_create_unauthorized_error() {
    let err = CustomError::UnAuthorized;
    debug_assert_eq!(err.to_string(), "UnAuthorized");
}

#[test]
pub fn should_create_bad_request_error() {
    let err = CustomError::BadRequest(String::from("hello world"));
    debug_assert_eq!(err.to_string(), "hello world");
}

#[test]
pub fn should_create_other_error() {
    let err = CustomError::Other;
    debug_assert_eq!(err.to_string(), "some custom message");
}

#[test]
pub fn should_create_error() {
    let err = CustomError::Other.to_error();
    debug_assert_eq!(err.to_string(), "some custom message");
    debug_assert_eq!(err.message(), Some("some custom message"));
    debug_assert_eq!(err.name(), Some("CustomError"));
}
