use crate::{Config, Key, Path};

#[derive(Debug, Clone)]
pub struct Section<Value: Config + Clone> {
    key: Key,
    path: Path,
    value: Value,
}

impl<Value: Config + Clone> Config for Section<Value> {
    fn value(&self) -> String {
        return self.value.value();
    }
}