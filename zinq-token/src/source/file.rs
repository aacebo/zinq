use zinq_error::{AnyError, BadRequestError, NotFoundError, Result};

///
/// ## File
/// container for the contents of a source file
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    path: Box<str>,
    name: Box<str>,
    ext: Box<str>,
    bytes: Vec<u8>,
}

impl File {
    #[inline]
    pub fn read(path: &str) -> Result<Self> {
        let p = std::path::Path::new(path);

        if !p.exists() {
            return Err(NotFoundError::from(format!("file '{}' not found", path)).into());
        }

        if !p.is_file() {
            return Err(BadRequestError::from("must be a file").into());
        }

        let bytes = match std::fs::read(path) {
            Err(err) => return Err(AnyError::new(err).into()),
            Ok(v) => v,
        };

        Ok(Self {
            path: Box::<str>::from(path.to_string()),
            name: Box::<str>::from(p.file_name().unwrap().to_string_lossy().to_string()),
            ext: Box::<str>::from(p.extension().unwrap().to_string_lossy().to_string()),
            bytes,
        })
    }

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

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
