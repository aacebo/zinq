use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context {
    fn get(&self, key: &str) -> Result<&dyn ToString>;
}

pub trait Section: Sized {
    fn from_context(context: &dyn Context) -> Result<Self>;
}

#[cfg(test)]
mod test {
    use crate::error::NotFoundError;

    use super::{Context, Result};

    struct CustomContext {
        host: String,
        port: usize,
    }

    impl Context for CustomContext {
        fn get(&self, key: &str) -> Result<&dyn ToString> {
            match key {
                "host" => Ok(&self.host),
                "port" => Ok(&self.port),
                v => Err(NotFoundError::from(format!("key '{}' not found", v)).into()),
            }
        }
    }

    #[test]
    pub fn should_create_context() {
        let context = CustomContext {
            host: String::from("test"),
            port: 3000,
        };

        let host = context.get("host").unwrap();
        let port = context.get("port").unwrap();

        debug_assert_eq!(host.to_string(), "test");
        debug_assert_eq!(port.to_string(), "3000");
    }
}
