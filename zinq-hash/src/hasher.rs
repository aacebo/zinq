use crate::{Hash, Object, Version};

#[derive(Debug, Clone)]
pub struct Hasher(blake3::Hasher);

impl Hasher {
    pub fn new() -> Self {
        Self::v1()
    }

    pub fn v1() -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(Version::V1.to_string().as_bytes());
        Self(hasher)
    }

    pub fn push<T: Hash>(&mut self, value: &T) -> &mut Self {
        self.0.update(b"::");
        value.hash(self);
        self
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(b"::");
        self.0.update(bytes);
        self
    }

    pub fn push_str(&mut self, input: &str) -> &mut Self {
        self.0.update(b"::");
        self.0.update(input.as_bytes());
        self
    }

    pub fn push_field<T: Hash + std::fmt::Display>(&mut self, name: &str, value: T) -> &mut Self {
        self.0.update(b"::");
        self.0.update(format!("{}={}", name, value).as_bytes());
        self
    }

    pub fn build(&self) -> Object {
        Object::from(self.0.finalize())
    }
}

impl From<blake3::Hasher> for Hasher {
    fn from(value: blake3::Hasher) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Hasher;

    #[test]
    fn should_have_attributes() {
        let object = Hasher::new()
            .push_str("test")
            .push_field("hello", 1u8)
            .build();

        debug_assert_eq!(
            object.to_string(),
            "be73d34c9a0cecab1035b870b644fdb4b105da02f4ff78b2b29839e745304497"
        );
    }
}
