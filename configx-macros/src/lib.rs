mod render;

use macrox::Element;
use proc_macro::TokenStream;

#[proc_macro_derive(Config)]
pub fn derive(tokens: TokenStream) -> TokenStream {
    let element = render::StructDerive;
    let mut context = match element.parse_derive(tokens.into()) {
        Err(err) => return err.to_compile_error().into(),
        Ok(v) => v,
    };

    return match element.render(&mut context) {
        Err(err) => err.to_compile_error().into(),
        Ok(tokens) => tokens.into(),
    };
}
