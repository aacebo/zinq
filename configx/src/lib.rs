mod key;
mod path;
mod get;
mod section;
pub mod types;

use std::sync::Arc;

pub use section::*;
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
    /// ## value
    /// the configs raw serialized value
    /// 
    fn value(&self) -> String;

    ///
    /// ## get
    /// get the raw value of a given path
    ///
    fn get(&self, path: &Path) -> Option<String>;

    ///
    /// ## section
    /// get a section at a given path
    ///
    fn section<'a>(&self, _path: &Path) -> Option<&'a dyn Section> {
        None
    }

    ///
    /// ## children
    /// the child config slice
    ///
    fn children(&self) -> Vec<Arc<dyn Section>> {
        vec![]
    }
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
    /// ## get_as
    /// get a value at some path
    ///
    fn get_as<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Option<T>;

    ///
    /// ## get_as_required
    /// get a value at some path, or panic
    ///
    fn get_as_required<T: serde::de::DeserializeOwned>(&self, path: &Path) -> T {
        match self.get_as(path) {
            None => panic!("[error] => value not found for '{}'", path,),
            Some(v) => v,
        }
    }
}

impl std::ops::Index<&str> for dyn Config {
    type Output = dyn Section;
    
    fn index(&self, index: &str) -> &Self::Output {
        let path = Path::from(index);
        self.section(&path).unwrap()
    }
}

#[cfg(feature = "yml")]
impl GetAs for dyn Config {
    fn get_as<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Option<T> {
        serde_yml::from_str(&self.get(path)?).unwrap_or(None)
    }
}
