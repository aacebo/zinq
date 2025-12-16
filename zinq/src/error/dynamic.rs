use std::sync::Arc;

use super::Error;

#[derive(Debug, Clone)]
pub struct DynError(Arc<dyn std::error::Error>);

impl DynError {
    pub fn new<T: std::error::Error + 'static>(error: T) -> Self {
        Self(Arc::new(error))
    }
}

impl Into<Error> for DynError {
    fn into(self) -> Error {
        Error::Dyn(self)
    }
}

impl std::fmt::Display for DynError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl std::error::Error for DynError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl std::ops::Deref for DynError {
    type Target = dyn std::error::Error;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::error::DynError;

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
        let err = DynError::new(CustomError::from("test"));

        debug_assert_eq!(err.to_string(), "test");
    }
}
