mod key;
mod meta;
mod path;

pub use key::*;
pub use meta::*;
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
    /// ## name
    /// the configs name
    ///
    fn name(&self) -> &str;

    ///
    /// ## children
    /// the child config slice
    ///
    fn iter(&self) -> std::vec::IntoIter<(Key, Box<dyn Config>)>;

    ///
    /// ## get
    /// get a child config by its key
    ///
    fn get(&self, key: &Key) -> Option<Box<dyn Config>>;

    ///
    /// ## get_or_panic
    /// get a child config by its key
    /// panic if not found
    ///
    fn get_or_panic(&self, key: &Key) -> Box<dyn Config> {
        match self.get(key) {
            None => panic!("[error] => '{}' not found in ", key),
            Some(v) => v,
        }
    }
}

impl<T> Config for T
where
    T: IntoIterator + Clone,
    T::Item: Sized + Config + std::any::Any,
{
    fn name(&self) -> &str {
        return std::any::type_name::<T>();
    }

    fn iter(&self) -> std::vec::IntoIter<(Key, Box<dyn Config>)> {
        return vec![].into_iter();
    }

    fn get(&self, key: &Key) -> Option<Box<dyn Config>> {
        match key {
            Key::Index(i) => Some(Box::new(self.clone().into_iter().nth(*i)?)),
            _ => None,
        }
    }
}
