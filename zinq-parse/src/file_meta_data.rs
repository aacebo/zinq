use zinq_error::{AnyError, BadRequestError, Error, NotFoundError, Result};

use crate::Bytes;

///
/// ## FileMetaData
/// attributes of a file describing
/// its format and location
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FileMetaData {
    path: Box<str>,
    name: Box<str>,
    ext: Box<str>,
}

impl FileMetaData {
    #[inline]
    pub fn read(&self) -> Result<Bytes> {
        let p = std::path::Path::new(self.path.as_ref());

        if !p.exists() {
            return Err(NotFoundError::from(format!("file '{}' not found", self.path)).into());
        }

        if !p.is_file() {
            return Err(BadRequestError::from("must be a file").into());
        }

        let bytes = match std::fs::read(self.path.as_ref()) {
            Err(err) => return Err(AnyError::new(err).into()),
            Ok(v) => v,
        };

        Ok(Bytes::from(bytes))
    }
}

impl TryFrom<&str> for FileMetaData {
    type Error = Error;

    #[inline]
    fn try_from(path: &str) -> std::result::Result<Self, Self::Error> {
        let p = std::path::Path::new(path);

        if !p.exists() {
            return Err(NotFoundError::from(format!("file '{}' not found", path)).into());
        }

        if !p.is_file() {
            return Err(BadRequestError::from("must be a file").into());
        }

        Ok(Self {
            path: Box::<str>::from(path.to_string()),
            name: Box::<str>::from(p.file_name().unwrap().to_string_lossy().to_string()),
            ext: Box::<str>::from(p.extension().unwrap().to_string_lossy().to_string()),
        })
    }
}

impl FileMetaData {
    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn ext(&self) -> &str {
        &self.ext
    }
}

impl std::fmt::Display for FileMetaData {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path)
    }
}
