use proc_macro2::TokenStream;

pub trait Derive {
    fn parse(tokens: TokenStream) -> TokenStream;
    fn try_parse(tokens: TokenStream) -> Result<TokenStream, crate::Error>;
}

pub trait Attribute {
    fn parse(input: TokenStream, tokens: TokenStream) -> TokenStream;
    fn try_parse(input: TokenStream, tokens: TokenStream) -> Result<TokenStream, crate::Error>;
}
