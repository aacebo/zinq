use proc_macro2::TokenStream;

pub trait StructElement {
    fn render(&self, context: &mut StructContext) -> Result<TokenStream, &dyn std::error::Error>;
}

pub struct StructContext {
    args: TokenStream,
    value: syn::ItemStruct,
}

impl StructContext {
    pub fn args<Args: syn::parse::Parse + quote::ToTokens>(&self) -> Result<Args, crate::Error> {
        return match syn::parse::<Args>(self.args.clone().into()) {
            Ok(args) => Ok(args),
            Err(err) => Err(crate::Error::new(
                "failed while attempting to parse struct macro arguments",
            )
            .with_inner_error(err)),
        };
    }

    pub fn value(&self) -> &syn::ItemStruct {
        return &self.value;
    }

    pub fn value_mut(&mut self) -> &mut syn::ItemStruct {
        return &mut self.value;
    }
}
