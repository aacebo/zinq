mod elements;
mod input;

use elements::*;
use input::*;
use proc_macro::TokenStream;
use macrox::Element;

#[proc_macro_attribute]
pub fn extend(input_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let element = StructExtend;
    let mut context = match element.parse(Some(input_tokens.into()), item_tokens.into()) {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    return match element.render(&mut context) {
        Err(err) => err.to_compile_error().into(),
        Ok(tokens) => tokens.into(),
    };
}
