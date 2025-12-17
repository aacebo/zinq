use std::sync::Arc;

use super::Error;

#[derive(Debug, Clone)]
pub struct AnyError(Arc<dyn std::error::Error>);

impl AnyError {
    pub fn new<T: std::error::Error + 'static>(error: T) -> Self {
        Self(Arc::new(error))
    }
}

impl Into<Error> for AnyError {
    fn into(self) -> Error {
        Error::Other(self)
    }
}

impl std::fmt::Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl std::ops::Deref for AnyError {
    type Target = dyn std::error::Error;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::error::ToError;

    #[derive(Debug)]
    struct CustomError(String);

    impl From<&str> for CustomError {
        fn from(value: &str) -> Self {
            Self(value.to_string())
        }
    }

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }

    impl std::error::Error for CustomError {}

    #[test]
    pub fn should_create_dyn_error() {
        let err = CustomError::from("test").to_error();
        debug_assert_eq!(err.to_string(), "test");
    }
}
