use zinq_error::{BAD_ARGUMENTS, Error, NOT_FOUND, Result, ZinqError, ZinqErrorCode};

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
            return Err(Error::from_str(&format!("file '{}' not found", self.path))
                .code(ZinqErrorCode {
                    id: 0,
                    name: "NotFound",
                    description: "Not Found",
                })
                .build()
                .into());
        }

        if !p.is_file() {
            return Err(Error::from_str("must be a file")
                .code(ZinqErrorCode {
                    id: 1,
                    name: "BadArguments",
                    description: "Bad Arguments",
                })
                .build()
                .into());
        }

        let bytes = match std::fs::read(self.path.as_ref()) {
            Err(err) => return Err(Error::from_error(err).build().into()),
            Ok(v) => v,
        };

        Ok(Bytes::from(bytes))
    }
}

impl TryFrom<&str> for FileMetaData {
    type Error = ZinqError;

    #[inline]
    fn try_from(path: &str) -> std::result::Result<Self, Self::Error> {
        let p = std::path::Path::new(path);

        if !p.exists() {
            return Err(Error::from_str(&format!("file '{}' not found", path))
                .code(NOT_FOUND)
                .build()
                .into());
        }

        if !p.is_file() {
            return Err(Error::from_str("must be a file")
                .code(BAD_ARGUMENTS)
                .build()
                .into());
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
