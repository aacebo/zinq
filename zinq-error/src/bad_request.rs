use super::Error;

#[derive(Debug, Clone)]
pub struct BadRequestError {
    message: Option<String>,
}

impl BadRequestError {
    pub fn new() -> Self {
        Self { message: None }
    }
}

impl From<String> for BadRequestError {
    fn from(value: String) -> Self {
        Self {
            message: Some(value),
        }
    }
}

impl From<&str> for BadRequestError {
    fn from(value: &str) -> Self {
        Self {
            message: Some(value.to_string()),
        }
    }
}

impl Into<Error> for BadRequestError {
    fn into(self) -> Error {
        Error::BadRequest(self)
    }
}

impl std::fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self.message {
            None => "not found",
            Some(v) => v.as_str(),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for BadRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::ops::Deref for BadRequestError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match &self.message {
            None => "not found",
            Some(v) => v.as_str(),
        }
    }
}
