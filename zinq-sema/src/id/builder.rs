use crate::id::{Attr, FieldAttr, Version};

pub struct Builder {
    version: Version,
    attrs: Vec<Attr>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            version: Version::default(),
            attrs: vec![],
        }
    }

    pub fn version(mut self, ver: Version) -> Self {
        self.version = ver;
        self
    }

    pub fn attr(mut self, value: Attr) -> Self {
        self.attrs.push(value);
        self
    }

    pub fn field(mut self, name: &str, value: Attr) -> Self {
        self.attrs.push(Attr::Field(FieldAttr {
            name: name.to_string(),
            value: Box::new(value),
        }));

        self
    }

    pub fn build(self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.version.to_string().as_bytes());

        for attr in self.attrs.iter() {
            hasher.update(attr.to_string().as_bytes());
        }

        hasher.finalize()
    }
}

impl std::fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)?;

        for attr in self.attrs.iter() {
            write!(f, "::{}", attr)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::id;

    #[test]
    fn should_have_default_version() {
        let id = id::Builder::new();
        debug_assert_eq!(id.to_string(), "V1");
    }

    #[test]
    fn should_have_attributes() {
        let id = id::Builder::new()
            .attr("test".into())
            .field("hello", 1u8.into());

        debug_assert_eq!(id.to_string(), "V1::test::hello=1");
        debug_assert_eq!(
            id.build().to_hex().to_string(),
            "c9ddde135203dc8e34d29a626064387f46779ab1be56693361abf9503f3933c0"
        );
    }
}
