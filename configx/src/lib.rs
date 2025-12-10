mod error;
mod get;
mod item;
mod key;
mod path;

use std::ops::Index;

pub use error::*;
pub use get::*;
pub use item::*;
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
    /// ## item
    /// get a config item by its key
    ///
    fn item(&self, key: &Key) -> Option<Item>;

    ///
    /// ## children
    /// the child config slice
    ///
    fn children(&self) -> Vec<Item> {
        vec![]
    }
}

impl GetByPath for dyn Config {
    fn get<T>(&self, path: &Path) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let key = match path.first() {
            None => return None,
            Some(v) => v,
        };

        let item = self.item(key)?;
        item.get(path.clone().pop())
    }
}

impl<T> GetByPath for T
where
    T: Config + Clone + 'static,
{
    fn get<V>(&self, path: &Path) -> Option<V>
    where
        V: serde::de::DeserializeOwned,
    {
        let key = path.last()?;
        let mut item = Item::Section(Section::new(Key::from(""), self.clone()));

        for i in 0..path.len() - 1 {
            item = item.item(path.index(i))?;
        }

        if let Some(Item::Value(v)) = item.item(key) {
            return Some(v.get().expect("expected error free deserialization"));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::{Config, GetByPath, Item, Key, Section, Value, path};

    #[derive(Default, Clone)]
    struct AppConfig {
        name: String,
        api: ApiConfig,
    }

    impl Config for AppConfig {
        fn item(&self, key: &Key) -> Option<Item> {
            let item = match key.to_string().as_str() {
                "name" => Item::Value(Value::new(key.clone(), self.name.clone())),
                "api" => Item::Section(Section::new(key.clone(), self.api.clone())),
                _ => return None,
            };

            Some(item)
        }
    }

    #[derive(Default, Clone)]
    struct ApiConfig {
        host: String,
        port: usize,
    }

    impl Config for ApiConfig {
        fn item(&self, key: &Key) -> Option<Item> {
            let item = match key.to_string().as_str() {
                "host" => Item::Value(Value::new(key.clone(), self.host.clone())),
                "port" => Item::Value(Value::new(key.clone(), self.port)),
                _ => return None,
            };

            Some(item)
        }
    }

    #[test]
    pub fn should_get_section() {
        let app = AppConfig {
            name: String::from("test"),
            api: ApiConfig {
                host: String::from("google"),
                port: 3000,
            },
        };

        debug_assert_eq!(app.get(&path!(/api/host)), Some(app.api.host.clone()));
        debug_assert_eq!(
            app.get::<String>(&path!(/api/host)).unwrap_or_default(),
            app.api.host.as_str()
        );
    }
}
