use quote::quote;
use zinq_parse::element::{StructContext, StructElement};

pub struct MyElement;

impl StructElement for MyElement {
    fn render(
        &self,
        context: &mut StructContext,
    ) -> Result<proc_macro2::TokenStream, &dyn std::error::Error> {
        Ok(quote! {})
    }
}
