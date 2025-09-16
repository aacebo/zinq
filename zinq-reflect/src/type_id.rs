#[derive(Debug, Clone, PartialEq)]
pub struct TypeId {
    inner: String,
}

impl TypeId {
    pub fn of<T>() -> Self {
        return Self {
            inner: String::new(),
        };
    }
}
