mod hasher;
mod object;
mod version;

pub use hasher::*;
pub use object::*;
pub use version::*;

pub trait Hash {
    fn hash(&self, hasher: &mut Hasher);
}

impl<T: ToString> Hash for T {
    fn hash(&self, hasher: &mut Hasher) {
        hasher.push_str(&self.to_string());
    }
}
