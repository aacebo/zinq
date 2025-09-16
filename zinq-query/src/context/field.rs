#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldContext<P, T> {
    pub query: crate::Query,
    pub parent: Box<P>,
    pub key: String,
    pub value: Option<T>,
}
