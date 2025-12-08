use std::collections::VecDeque;

use crate::Key;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Path(VecDeque<Key>);

impl Path {
    pub fn new() -> Self {
        return Self(VecDeque::new());
    }

    pub fn depth(&self) -> usize {
        return self.0.len();
    }

    pub fn key(&self) -> Option<&Key> {
        return self.0.back();
    }

    pub fn push(&self, segment: Key) -> Self {
        let mut next = self.clone();
        next.0.push_back(segment);
        return next;
    }

    pub fn pop(&mut self) -> Option<Key> {
        return self.0.pop_front();
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut segments = VecDeque::new();

        for part in value.split("/") {
            segments.push_back(Key::from(part));
        }

        Self(segments)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut segments = VecDeque::new();

        for part in value.split("/") {
            segments.push_back(Key::from(part));
        }

        Self(segments)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, segment) in self.0.iter().enumerate() {
            write!(f, "{}", segment)?;

            if i < self.0.len() - 1 {
                write!(f, "/")?;
            }
        }

        Ok(())
    }
}

impl PartialEq<str> for Path {
    fn eq(&self, other: &str) -> bool {
        return self.to_string() == other;
    }
}
