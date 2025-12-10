mod iter;

pub use iter::*;

use std::collections::VecDeque;

use crate::Key;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Path(VecDeque<Key>);

impl Path {
    pub fn new() -> Self {
        return Self(VecDeque::new());
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn first(&self) -> Option<&Key> {
        return self.0.front();
    }

    pub fn last(&self) -> Option<&Key> {
        return self.0.back();
    }

    pub fn push(&mut self, key: Key) -> &mut Self {
        self.0.push_back(key);
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
        let mut left: VecDeque<Key> = self.0.clone();
        let right = left.split_off(index);
        (Path::from(left), Path::from(right))
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut keys = VecDeque::new();

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            keys.push_back(Key::from(part));
        }

        Self(keys)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut keys = VecDeque::new();

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            keys.push_back(Key::from(part));
        }

        Self(keys)
    }
}

impl From<Vec<Key>> for Path {
    fn from(value: Vec<Key>) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<VecDeque<Key>> for Path {
    fn from(value: VecDeque<Key>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/")?;

        for (i, key) in self.iter().enumerate() {
            write!(f, "{}", key)?;

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
    type Output = Key;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl std::ops::IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[macro_export]
macro_rules! path {
    ($($path:tt)+) => {
        $crate::Path::from(stringify!($($path)+))
    };
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
