use crate::Path;

///
/// ## GetByPath
/// a config that can access its value
///
pub trait GetByPath {
    ///
    /// ## get
    /// get a value at some path
    ///
    fn get<T>(&self, path: &Path) -> Option<T>
    where
        T: serde::de::DeserializeOwned;

    ///
    /// ## get_required
    /// get a value at some path, or panic
    ///
    fn get_required<T>(&self, path: &Path) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        match self.get(path) {
            None => panic!("[error] => value not found for '{}'", path,),
            Some(v) => v,
        }
    }
}
