mod context;
mod error;
mod parse;
mod resolve;
mod result;

#[cfg(feature = "macros")]
pub use zinq_query_macros::*;

pub use context::*;
pub use error::*;
pub use resolve::*;
pub use result::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Query {
    pub args: BTreeMap<String, zinq_reflect::Value>,
    pub fields: BTreeMap<String, Query>,
}
