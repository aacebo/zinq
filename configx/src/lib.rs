mod key;
mod path;
pub mod types;

use std::sync::Arc;

pub use key::*;
pub use path::*;

#[cfg(feature = "macros")]
pub use configx_macros::*;

///
/// ## Config
/// base config contract, all other configs
/// are derived from this
///
pub trait Config {
    ///
    /// ## get
    /// get the raw value of a given path
    ///
    fn get(&self, path: &Path) -> Option<&str>;

    ///
    /// ## section
    /// get a section at a given path
    ///
    fn section(&self, path: &Path) -> Option<Arc<dyn Section>>;

    ///
    /// ## children
    /// the child config slice
    ///
    fn children(&self) -> Vec<Arc<dyn Section>>;
}

///
/// ## Section
/// a config that is a child section to some parent
/// configuration and may also have sub configs
///
pub trait Section: Config {
    ///
    /// ## path
    /// the absolute path from the root
    /// config to this section
    ///
    fn path(&self) -> Path;

    ///
    /// ## key
    /// the key this section occupies in its parent.
    ///
    fn key(&self) -> Key;
}

///
/// ## Value
/// a config that can access its value
///
pub trait GetAs: Config {
    ///
    /// ## get
    /// get a value at some path
    ///
    fn get_as<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Option<T>;

    ///
    /// ## get_required
    /// get a value at some path, or panic
    ///
    fn get_as_required<T: serde::de::DeserializeOwned>(&self, path: &Path) -> T {
        match self.get_as(path) {
            None => panic!("[error] => value not found for '{}'", path,),
            Some(v) => v,
        }
    }
}

#[cfg(feature = "yml")]
impl GetAs for dyn Config {
    fn get_as<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Option<T> {
        serde_yml::from_str(self.get(path)?)
            .expect(&format!("could not deserialize config path '{}'", path))
    }
}
