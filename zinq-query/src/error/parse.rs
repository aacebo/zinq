use crate::parse::Position;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseError {
    pub position: Position,
    pub message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        return Self {
            position: Position::default(),
            message: message.to_string(),
        };
    }

    pub fn position(self, position: Position) -> Self {
        let mut clone = self.clone();
        clone.position = position;
        return clone;
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[{}] => {}", self.position, self.message);
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
