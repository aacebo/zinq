use zinq_error::Result;

use crate::source::File;

///
/// ## Map
/// a mapping of source files that can
/// load/cache/add source files
///
#[derive(Debug, Clone)]
pub struct Map {
    files: Vec<File>,
}

impl Map {
    #[inline]
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, File> {
        self.files.iter()
    }

    #[inline]
    pub fn get(&self, path: &str) -> Option<&File> {
        self.files.iter().find(|file| file.path() == path)
    }

    #[inline]
    pub fn add(&mut self, path: &str) -> Result<&File> {
        let file = File::read(path)?;
        self.files.push(file);
        Ok(self.files.last().unwrap())
    }
}

impl std::ops::Deref for Map {
    type Target = [File];

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl std::ops::Index<usize> for Map {
    type Output = File;

    fn index(&self, index: usize) -> &Self::Output {
        self.files.index(index)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in &self.files {
            write!(f, "{}", file.path())?;
        }

        Ok(())
    }
}
