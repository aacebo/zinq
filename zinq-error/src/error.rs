use std::rc::Rc;

use crate::{ErrorBuilder, StdError, ZinqError, ZinqErrorCode};

///
/// ## Error
/// a basic error implementation that
/// can be used standalone or wrapped
/// by custom error types.
///
#[derive(Debug, Default, Clone)]
pub struct Error {
    pub code: ZinqErrorCode,
    pub message: Option<String>,
    pub source: Option<Rc<dyn StdError>>,
    pub children: Vec<ZinqError>,
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
        write!(f, "[{}] => ", &self.code.id)?;

        if let Some(message) = &self.message {
            write!(f, "{}", message)?;
        }

        if let Some(src) = &self.source {
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
            Some(v) => Some(v.as_ref()),
        }
    }
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        if &self.code != &other.code {
            return false;
        }

        if &self.message != &other.message {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
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
        let err = CustomError::from("test");
        debug_assert_eq!(err.to_string(), "test"); // [0] because no category
    }
}
