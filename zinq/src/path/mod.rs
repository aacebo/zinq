mod iter;
mod segment;

pub use iter::*;
pub use segment::*;

use std::collections::VecDeque;

#[macro_export]
macro_rules! path {
    ($($path:tt)+) => {
        $crate::path::Path::from(stringify!($($path)+))
    };
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Path(VecDeque<Segment>);

impl Path {
    pub fn new() -> Self {
        return Self(VecDeque::new());
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn first(&self) -> Option<&Segment> {
        return self.0.front();
    }

    pub fn last(&self) -> Option<&Segment> {
        return self.0.back();
    }

    pub fn push(&mut self, segment: Segment) -> &mut Self {
        self.0.push_back(segment);
        return self;
    }

    pub fn pop(&mut self) -> &mut Self {
        self.0.pop_front();
        return self;
    }

    pub fn iter(&self) -> Iter<'_> {
        return Iter::from(self);
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        let mut left: VecDeque<Segment> = self.0.clone();
        let right = left.split_off(index);
        (Path::from(left), Path::from(right))
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut segments = VecDeque::new();

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            segments.push_back(Segment::from(part));
        }

        Self(segments)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut segments = VecDeque::new();

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            segments.push_back(Segment::from(part));
        }

        Self(segments)
    }
}

impl From<Vec<Segment>> for Path {
    fn from(value: Vec<Segment>) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<VecDeque<Segment>> for Path {
    fn from(value: VecDeque<Segment>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/")?;

        for (i, segment) in self.iter().enumerate() {
            write!(f, "{}", segment)?;

            if i < self.0.len() - 1 {
                write!(f, "/")?;
            }
        }

        Ok(())
    }
}

impl PartialEq<&str> for Path {
    fn eq(&self, other: &&str) -> bool {
        return self.to_string() == *other;
    }
}

impl std::ops::Index<usize> for Path {
    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl std::ops::IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn should_create_path_from_tokens() {
        let path = path!(/a/0/test);

        debug_assert_eq!(path.len(), 3);
        debug_assert_eq!(path[0], "a");
        debug_assert_eq!(path[1], 0);
        debug_assert_eq!(path[2], "test");
        debug_assert_eq!(path, "/a/0/test", "{}", path);
    }
}
