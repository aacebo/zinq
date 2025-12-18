use super::Error;

#[derive(Debug, Clone)]
pub struct NotFoundError {
    message: Option<String>,
}

impl NotFoundError {
    pub fn new() -> Self {
        Self { message: None }
    }
}

impl From<String> for NotFoundError {
    fn from(value: String) -> Self {
        Self {
            message: Some(value),
        }
    }
}

impl From<&str> for NotFoundError {
    fn from(value: &str) -> Self {
        Self {
            message: Some(value.to_string()),
        }
    }
}

impl Into<Error> for NotFoundError {
    fn into(self) -> Error {
        Error::NotFound(self)
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self.message {
            None => "not found",
            Some(v) => v.as_str(),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for NotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::ops::Deref for NotFoundError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match &self.message {
            None => "not found",
            Some(v) => v.as_str(),
        }
    }
}
