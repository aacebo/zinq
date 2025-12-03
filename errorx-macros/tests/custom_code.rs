use errorx::ToError;
use errorx_macros::*;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error(code = 1234)]
    UnAuthorized,
}

#[test]
pub fn should_create_error() {
    let err = CustomError::UnAuthorized.to_error();
    debug_assert_eq!(err.to_string(), "UnAuthorized");
    debug_assert_eq!(err.message(), None);
    debug_assert_eq!(err.name(), Some("UnAuthorized"));
    debug_assert_eq!(err.code(), Some(1234));
}
