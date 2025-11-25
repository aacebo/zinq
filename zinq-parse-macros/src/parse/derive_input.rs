#[derive(Debug, Clone)]
pub struct DeriveInput {
    pub context: Option<syn::TypePath>,
    pub output: Option<syn::TypePath>,
}

impl syn::parse::Parse for DeriveInput {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut context: Option<syn::TypePath> = None;
        let mut output: Option<syn::TypePath> = None;

        while stream.peek(syn::Lit) {
            let ident = stream.parse::<syn::LitStr>()?;
            stream.parse::<syn::Token![=]>()?;
            let path = stream.parse::<syn::TypePath>()?;

            if ident.value() == "Context" {
                context = Some(path);
            } else if ident.value() == "Output" {
                output = Some(path);
            }
        }

        Ok(Self {
            context,
            output,
        })
    }
}