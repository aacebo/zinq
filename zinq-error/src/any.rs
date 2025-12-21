use std::sync::Arc;

use super::Error;

#[derive(Debug, Clone)]
pub struct AnyError {
    code: Option<u32>,
    inner: Arc<dyn std::error::Error>,
}

impl AnyError {
    pub fn new<T: std::error::Error + 'static>(error: T) -> Self {
        Self {
            code: None,
            inner: Arc::new(error),
        }
    }

    pub fn with_code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn code(&self) -> Option<u32> {
        self.code
    }
}

impl From<Error> for AnyError {
    fn from(value: Error) -> Self {
        Self {
            code: value.code(),
            inner: Arc::new(value),
        }
    }
}

impl Into<Error> for AnyError {
    fn into(self) -> Error {
        Error::Any(self)
    }
}

impl std::fmt::Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "[{}] => ", code)?;
        }

        write!(f, "{}", &self.inner)
    }
}

impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl std::ops::Deref for AnyError {
    type Target = dyn std::error::Error;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::ToError;

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
