mod parse;
mod render;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn zinq_derive(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(args as parse::Input);
    let mut target = syn::parse_macro_input!(tokens as syn::Item);

    match render::element(&input, &mut target) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out.into(),
    }
}
