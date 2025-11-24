use crate::LazyParse;

#[derive(Clone)]
pub struct TypeEntry {
    pub declare: LazyParse<syn::Item>,
    // pub impls: Vec<LazyParse<syn::ItemImpl>>,
}

impl From<syn::Item> for TypeEntry {
    fn from(value: syn::Item) -> Self {
        return Self {
            declare: LazyParse::from(value),
            // impls: vec![],
        };
    }
}
