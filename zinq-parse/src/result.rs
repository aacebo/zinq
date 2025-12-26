use crate::Diagnostic;

#[derive(Debug, Clone)]
pub struct ParseResult<T> {
    pub value: T,
    pub diagnostic: Option<Diagnostic>,
}

impl<T> ParseResult<T> {
    pub fn from_value(value: T) -> Self {
        Self {
            value: value,
            diagnostic: None,
        }
    }
}
