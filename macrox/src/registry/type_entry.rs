use syn::spanned::Spanned;

use crate::LazyParse;

#[derive(Clone)]
pub struct TypeEntry {
    pub declare: LazyParse<syn::Item>,
    pub impls: Vec<LazyParse<syn::ItemImpl>>,
}

impl TypeEntry {
    pub fn span(&self) -> proc_macro2::Span {
        return self.declare.get().span();
    }

    pub fn fields(&self) -> Option<syn::Fields> {
        return match self.declare.get() {
            syn::Item::Struct(item) => Some(item.fields),
            _ => None,
        };
    }

    pub fn ident(&self) -> Option<syn::Ident> {
        return match self.declare.get() {
            syn::Item::Struct(item) => Some(item.ident),
            syn::Item::Enum(item) => Some(item.ident),
            syn::Item::Trait(item) => Some(item.ident),
            syn::Item::Mod(item) => Some(item.ident),
            syn::Item::Const(item) => Some(item.ident),
            syn::Item::ExternCrate(item) => Some(item.ident),
            syn::Item::Fn(item) => Some(item.sig.ident),
            syn::Item::Static(item) => Some(item.ident),
            syn::Item::TraitAlias(item) => Some(item.ident),
            syn::Item::Type(item) => Some(item.ident),
            syn::Item::Union(item) => Some(item.ident),
            syn::Item::Macro(item) => match item.ident {
                None => None,
                Some(ident) => Some(ident),
            },
            _ => None,
        };
    }

    pub fn attrs(&self) -> Vec<syn::Attribute> {
        return match self.declare.get() {
            syn::Item::Struct(item) => item.attrs,
            syn::Item::Trait(item) => item.attrs,
            syn::Item::Enum(item) => item.attrs,
            syn::Item::Fn(item) => item.attrs,
            syn::Item::Const(item) => item.attrs,
            syn::Item::Static(item) => item.attrs,
            syn::Item::ExternCrate(item) => item.attrs,
            syn::Item::ForeignMod(item) => item.attrs,
            syn::Item::Impl(item) => item.attrs,
            syn::Item::Macro(item) => item.attrs,
            syn::Item::TraitAlias(item) => item.attrs,
            syn::Item::Type(item) => item.attrs,
            syn::Item::Union(item) => item.attrs,
            syn::Item::Use(item) => item.attrs,
            syn::Item::Mod(item) => item.attrs,
            _ => vec![],
        };
    }
}

impl TypeEntry {
    pub fn with_impl(&self, value: syn::ItemImpl) -> Self {
        let mut next = self.clone();
        next.impls.push(LazyParse::from(value));
        return next;
    }
}

impl From<syn::Item> for TypeEntry {
    fn from(value: syn::Item) -> Self {
        return Self {
            declare: LazyParse::from(value),
            impls: vec![],
        };
    }
}
