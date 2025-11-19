use std::sync::Arc;

pub trait Value: std::fmt::Display {}

impl std::fmt::Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

impl<T: std::fmt::Display> Value for T {}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Option<String>,
    pub value: Arc<dyn Value>,
}
