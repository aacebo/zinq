use super::{Path, Segment};

#[derive(Debug, Clone)]
pub struct Iter<'a> {
    index: usize,
    inner: &'a Path,
}

impl<'a> Iter<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn left(&self) -> Path {
        let (left, _) = self.inner.split(self.index);
        left
    }

    pub fn right(&self) -> Path {
        let (_, right) = self.inner.split(self.index);
        right
    }

    pub fn seek(&mut self, segment: &Segment) -> Option<usize> {
        while let Some(v) = self.next() {
            if v == segment {
                self.index -= 1;
                return Some(self.index);
            }
        }

        None
    }
}

impl<'a> From<&'a Path> for Iter<'a> {
    fn from(value: &'a Path) -> Self {
        Self {
            index: 0,
            inner: value,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.0.get(self.index);
        self.index += 1;
        item
    }
}

#[cfg(test)]
mod test {
    use crate::path;

    #[test]
    pub fn should_iterate_path() {
        let path = path!(/a/b/c);

        for (i, segment) in path.iter().enumerate() {
            debug_assert_eq!(segment, &path[i]);
        }
    }

    #[test]
    pub fn should_split_path() {
        let path = path!(/a/b/1/c/d/e);
        let mut iter = path.iter();

        debug_assert_eq!(iter.seek(&crate::path::Segment::from(1)), Some(2));
        debug_assert_eq!(iter.index(), 2);
        debug_assert_eq!(iter.left(), path!(/a/b), "{}", iter.left().to_string());
        debug_assert_eq!(
            iter.right(),
            path!(/1/c/d/e),
            "{}",
            iter.right().to_string()
        );
    }
}
