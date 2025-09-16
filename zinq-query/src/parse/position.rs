#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub ln: usize,
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn ln(self, ln: usize) -> Self {
        let mut clone = self.clone();
        clone.ln = ln;
        return clone;
    }

    pub fn start(self, start: usize) -> Self {
        let mut clone = self.clone();
        clone.start = start;
        return clone;
    }

    pub fn end(self, end: usize) -> Self {
        let mut clone = self.clone();
        clone.end = end;
        return clone;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}, {} - {}", self.ln, self.start, self.end,);
    }
}
