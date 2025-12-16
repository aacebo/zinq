mod value;

pub use value::*;

use crate::error::Result;

pub trait Context {
    fn get(&self, key: &str) -> Result<Value>;
    fn set(&mut self, key: &str, value: Value) -> Result<()>;
}

pub trait Extract: Sized {
    fn extract(context: &dyn Context) -> Result<Self>;
}

#[cfg(test)]
mod test {
    use crate::{
        context::Value,
        error::{DynError, NotFoundError},
    };

    use super::{Context, Result};

    struct CustomContext {
        host: String,
        port: usize,
        disabled: bool,
    }

    impl Context for CustomContext {
        fn get(&self, key: &str) -> Result<Value> {
            match key {
                "host" => Ok(self.host.as_str().into()),
                "port" => Ok(self.port.into()),
                "disabled" => Ok(self.disabled.into()),
                v => Err(NotFoundError::from(format!("key '{}' not found", v)).into()),
            }
        }

        fn set(&mut self, key: &str, value: Value) -> Result<()> {
            if key == "host" {
                self.host = value.to_string();
                return Ok(());
            }

            if key == "port" {
                self.port = match value.to_string().parse() {
                    Err(err) => return Err(DynError::new(err).into()),
                    Ok(v) => v,
                };

                return Ok(());
            }

            if key == "disabled" {
                self.disabled = match value.to_string().parse() {
                    Err(err) => return Err(DynError::new(err).into()),
                    Ok(v) => v,
                };

                return Ok(());
            }

            Err(NotFoundError::from(format!("key '{}' not found", key)).into())
        }
    }

    #[test]
    pub fn should_create_context() {
        let context = CustomContext {
            host: String::from("test"),
            port: 3000,
            disabled: true,
        };

        let host = context.get("host").unwrap();
        let port = context.get("port").unwrap();
        let disabled = context.get("disabled").unwrap();

        debug_assert_eq!(&host, "test");
        debug_assert_eq!(&port, "3000");
        debug_assert_eq!(&disabled, "true");
    }
}
