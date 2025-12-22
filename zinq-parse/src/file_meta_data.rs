use zinq_error::{Error, Result};

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
            return Err(Error::not_found()
                .with_message(&format!("file '{}' not found", self.path))
                .build());
        }

        if !p.is_file() {
            return Err(Error::bad_arguments()
                .with_message("must be a file")
                .build());
        }

        let bytes = match std::fs::read(self.path.as_ref()) {
            Err(err) => return Err(Error::from(err).build()),
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
            return Err(Error::not_found()
                .with_message(&format!("file '{}' not found", path))
                .build());
        }

        if !p.is_file() {
            return Err(Error::bad_arguments()
                .with_message("must be a file")
                .build());
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
