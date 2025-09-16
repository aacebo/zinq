#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct BasicError(String);

impl BasicError {
    pub fn new(message: &str) -> Self {
        return Self(message.to_string());
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}
impl std::fmt::Display for BasicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl std::error::Error for BasicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
