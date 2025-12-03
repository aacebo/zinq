use errorx_macros::*;

pub mod custom_code;
pub mod custom_message;
pub mod custom_name;

#[derive(Error, Debug)]
pub enum CustomError {
    BadRequest(String),
    UnAuthorized,
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
