use std::sync::Arc;

use crate::{Builder, Code, StdError};

#[derive(Debug, Default, Clone)]
pub struct Error {
    pub(crate) code: Code,
    pub(crate) message: Option<String>,
    pub(crate) source: Option<Arc<dyn StdError>>,
    pub(crate) children: Vec<Self>,
}

impl Error {
    pub fn new() -> Builder {
        Builder::new()
    }

    pub fn from<Err: StdError>(error: Err) -> Builder {
        Builder::new().with_source(error)
    }

    pub fn from_str(message: &str) -> Builder {
        Builder::new().with_message(message)
    }

    pub fn not_found() -> Builder {
        Builder::new().with_code(Code::NotFound)
    }

    pub fn bad_arguments() -> Builder {
        Builder::new().with_code(Code::BadArguments)
    }
}

impl Error {
    pub fn code(&self) -> &Code {
        &self.code
    }

    pub fn message(&self) -> Option<&str> {
        match &self.message {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn source(&self) -> Option<&dyn StdError> {
        match &self.source {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn children(&self) -> &[Self] {
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
        if self.code != Code::Unknown {
            write!(f, "[{}] => ", &self.code)?;
        }

        if let Some(message) = self.message() {
            write!(f, "{}", message)?;
        } else if let Some(src) = self.source() {
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
        debug_assert_eq!(err.to_string(), "test");
    }
}
