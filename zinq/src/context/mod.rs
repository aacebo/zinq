mod value;

pub use value::*;

use crate::error::Result;

pub trait Context {
    fn parent(&self) -> Option<&dyn Context>;
    fn get(&self, key: &str) -> Result<Value>;
}

pub trait Section: Sized {
    const KEY: &'static str;

    fn extract(context: &dyn Context) -> Result<Self>;
}

pub trait Extract: Sized {
    fn extract(context: &dyn Context) -> Result<Self>;
}

#[cfg(test)]
mod test {
    use super::{Context, Result, Value};
    use crate::error::NotFoundError;

    struct CustomContext {
        parent: Option<Box<dyn Context>>,
        host: String,
        port: usize,
        disabled: bool,
    }

    impl Context for CustomContext {
        fn parent(&self) -> Option<&dyn Context> {
            match &self.parent {
                None => None,
                Some(v) => Some(v.as_ref()),
            }
        }

        fn get(&self, key: &str) -> Result<Value> {
            match key {
                "host" => Ok(self.host.as_str().into()),
                "port" => Ok(self.port.into()),
                "disabled" => Ok(self.disabled.into()),
                v => Err(NotFoundError::from(format!("key '{}' not found", v)).into()),
            }
        }
    }

    #[test]
    pub fn should_create_context() {
        let context = CustomContext {
            parent: None,
            host: String::from("test"),
            port: 3000,
            disabled: true,
        };

        let host = context.get("host").expect("expected host");
        let port = context.get("port").expect("expected port");
        let disabled = context.get("disabled").expect("expected disabled");

        debug_assert_eq!(&host, "test");
        debug_assert_eq!(&port, "3000");
        debug_assert_eq!(&disabled, "true");
    }
}
