use crate::Key;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Path(Vec<Key>);

impl Path {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn depth(&self) -> usize {
        return self.0.len();
    }

    pub fn key(&self) -> Option<&Key> {
        return self.0.last();
    }

    pub fn add(&self, segment: Key) -> Self {
        let mut next = self.clone();
        next.0.push(segment);
        return next;
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut segments = vec![];

        for part in value.split("/") {
            segments.push(Key::from(part));
        }

        Self(segments)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut segments = vec![];

        for part in value.split("/") {
            segments.push(Key::from(part));
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
