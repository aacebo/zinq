use syn::punctuated::Punctuated;

#[derive(Clone, Default)]
pub struct Input(pub Punctuated<syn::Ident, syn::Token![,]>);

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let extends: Punctuated<syn::Ident, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        return Ok(Self(extends));
    }
}
