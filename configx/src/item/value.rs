use crate::Key;

#[derive(Debug, Clone)]
pub struct Value {
    key: Key,
    value: String,
}

impl Value {
    pub fn new<T: ToString + 'static>(key: Key, value: T) -> Self {
        return Self {
            key,
            value: value.to_string(),
        };
    }

    pub fn key(&self) -> &Key {
        return &self.key;
    }

    pub fn get<T>(&self) -> Result<T, crate::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        if cfg!(feature = "yml") {
            return match serde_yml::from_str(self.value.as_str()) {
                Err(err) => Err(crate::Error::Yml(err)),
                Ok(v) => Ok(v),
            };
        }

        if cfg!(feature = "json") {
            return match serde_json::from_str(&self.value) {
                Err(err) => Err(crate::Error::Json(err)),
                Ok(v) => Ok(v),
            };
        }

        panic!("could not parse config value, no serialization feature found")
    }
}

impl std::ops::Deref for Value {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return self.value.as_ref();
    }
}
