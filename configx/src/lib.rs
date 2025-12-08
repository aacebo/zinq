mod item;
mod key;
mod path;

// pub use item::*;
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
    fn value<'a>(&self, key: &Key) -> Option<&'a dyn Section>;

    ///
    /// ## section
    /// get a section at a given path
    ///
    #[allow(unused_variables)]
    fn get<'a>(&self, path: &Path) -> Option<&'a dyn Section> {
        match path.first() {
            None => None,
            Some(key) => self.value(key),
        }
    }

    ///
    /// ## children
    /// the child config slice
    ///
    fn children<'a>(&self) -> Vec<&'a dyn Section> {
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
#[cfg(feature = "serde")]
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

// #[cfg(feature = "yml")]
// impl GetAs for dyn Config {
//     fn get_as<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Option<T> {
//         serde_yml::from_str(&self.get(path)?).unwrap_or(None)
//     }
// }

// impl std::ops::Index<&str> for dyn Config {
//     type Output = dyn Section;

//     fn index(&self, index: &str) -> &Self::Output {
//         let path = Path::from(index);
//         self.get(&path).unwrap()
//     }
// }

// impl std::ops::Index<usize> for dyn Config {
//     type Output = dyn Section;

//     fn index(&self, index: usize) -> &Self::Output {
//         let path = Path::from(format!("{}", index));
//         self.get(&path).unwrap()
//     }
// }
