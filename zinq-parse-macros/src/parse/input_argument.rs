#![allow(unused)]

use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct InputArgument<Value: Parse> {
    name: syn::Ident,
    eq: syn::token::Eq,
    value: Value,
}

impl<Value: Parse> InputArgument<Value> {
    pub fn name(&self) -> &syn::Ident {
        return &self.name;
    }

    pub fn eq(&self) -> &syn::token::Eq {
        return &self.eq;
    }

    pub fn value(&self) -> &Value {
        return &self.value;
    }
}

impl<Value: Parse> Parse for InputArgument<Value> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}
