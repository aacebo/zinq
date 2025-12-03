use std::marker::PhantomData;

use quote::ToTokens;
use syn::parse::Parse;

#[derive(Debug, Clone)]
pub struct LazyParse<T: Parse + ToTokens> {
    raw: String,
    __phantom__: PhantomData<T>,
}

impl<T: Parse + ToTokens> LazyParse<T> {
    pub fn get(&self) -> T {
        return syn::parse_str::<T>(&self.raw).expect("invalid tokens");
    }
}

impl<T: Parse + ToTokens> From<T> for LazyParse<T> {
    fn from(value: T) -> Self {
        return Self {
            raw: value.to_token_stream().to_string(),
            __phantom__: PhantomData,
        };
    }
}

impl<T: Parse + ToTokens> Parse for LazyParse<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<T>()?;

        return Ok(Self {
            raw: value.to_token_stream().to_string(),
            __phantom__: PhantomData,
        });
    }
}
