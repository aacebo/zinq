use std::sync::Arc;

use crate::{ErrorBuilder, ErrorCategory, ErrorCode, StdError, ZinqError};

///
/// ## Error
/// a basic error implementation that
/// can be used standalone or wrapped
/// by custom error types.
///
#[derive(Debug, Default, Clone)]
pub struct Error {
    pub(crate) code: ErrorCode,
    pub(crate) category: ErrorCategory,
    pub(crate) message: Option<String>,
    pub(crate) source: Option<Arc<dyn StdError>>,
    pub(crate) children: Vec<Self>,
}

impl Error {
    pub fn new() -> ErrorBuilder {
        ErrorBuilder::new()
    }

    pub fn from_error<Err: StdError>(error: Err) -> ErrorBuilder {
        ErrorBuilder::new().source(error)
    }

    pub fn from_str(message: &str) -> ErrorBuilder {
        ErrorBuilder::new().message(message)
    }
}

impl ZinqError for Error {
    fn code(&self) -> &ErrorCode {
        &self.code
    }

    fn category(&self) -> &ErrorCategory {
        &self.category
    }

    fn message(&self) -> Option<&str> {
        match &self.message {
            None => None,
            Some(v) => Some(v),
        }
    }

    fn source(&self) -> Option<&dyn StdError> {
        match &self.source {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    fn children(&self) -> &[Self] {
        &self.children
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self {
            message: Some(value),
            ..Default::default()
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            message: Some(value.to_string()),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}{}] => ", &self.category.id, &self.code.id)?;

        if let Some(message) = self.message() {
            write!(f, "{}", message)?;
        }

        if let Some(src) = self.source() {
            write!(f, "{}", src)?;
        }

        for (i, child) in self.children.iter().enumerate() {
            write!(f, "\n  - [{}] => {}", i, child)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            None => None,
            Some(v) => Some(v),
        }
    }
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        if self.code() != other.code() {
            return false;
        }

        if self.message() != other.message() {
            return false;
        }

        true
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
        debug_assert_eq!(err.to_string(), "[0] => test"); // [0] because no category
    }
}
