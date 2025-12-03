use proc_macro2::TokenStream;
use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct StructContext {
    pub(crate) input: Option<TokenStream>,
    pub(crate) target: syn::ItemStruct,
}

impl StructContext {
    pub fn input<T: Parse>(&self) -> Option<T> {
        return match &self.input {
            None => None,
            Some(tokens) => Some(syn::parse::<T>(tokens.clone().into()).unwrap()),
        };
    }

    pub fn target(&self) -> &syn::ItemStruct {
        return &self.target;
    }

    pub fn target_mut(&mut self) -> &mut syn::ItemStruct {
        return &mut self.target;
    }
}
