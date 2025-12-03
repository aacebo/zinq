pub trait ToError {
    fn to_error(&self) -> crate::Error;
}

impl std::fmt::Debug for dyn ToError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.to_error());
    }
}

impl std::fmt::Display for dyn ToError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_error());
    }
}

impl std::error::Error for dyn ToError {}
