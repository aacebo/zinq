use proc_macro2::TokenStream;
use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct EnumContext {
    pub(crate) input: Option<TokenStream>,
    pub(crate) target: syn::ItemEnum,
}

impl EnumContext {
    pub fn input<T: Parse>(&self) -> Option<T> {
        return match &self.input {
            None => None,
            Some(tokens) => Some(syn::parse::<T>(tokens.clone().into()).unwrap()),
        };
    }

    pub fn target(&self) -> &syn::ItemEnum {
        return &self.target;
    }

    pub fn target_mut(&mut self) -> &mut syn::ItemEnum {
        return &mut self.target;
    }
}
