use crate::Key;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Path(Vec<Key>);

impl Path {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn first(&self) -> Option<&Key> {
        return self.0.first();
    }

    pub fn last(&self) -> Option<&Key> {
        return self.0.last();
    }

    pub fn push(&mut self, key: Key) -> &mut Self {
        self.0.push(key);
        return self;
    }

    pub fn pop(&mut self) -> Option<Key> {
        return self.0.pop();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Key> {
        return self.0.iter();
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut keys = vec![];

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            keys.push(Key::from(part));
        }

        Self(keys)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        let mut keys = vec![];

        for part in value.split("/") {
            if part.is_empty() {
                continue;
            }

            keys.push(Key::from(part));
        }

        Self(keys)
    }
}

impl From<Vec<Key>> for Path {
    fn from(value: Vec<Key>) -> Self {
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
        debug_assert_eq!(path, "/a/0/test");
    }
}
