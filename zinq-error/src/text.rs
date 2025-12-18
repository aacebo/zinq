use super::Error;

#[derive(Debug, Clone)]
pub struct TextError(pub(crate) String);

impl From<String> for TextError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for TextError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Into<Error> for TextError {
    fn into(self) -> Error {
        Error::Text(self)
    }
}

impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::error::Error for TextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::ops::Deref for TextError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
