#[derive(Debug, Clone)]
pub struct Parameter {
    name: Option<String>,
    format: Option<String>,
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        }

        if let Some(format) = &self.format {
            write!(f, ":{}", format)?;
        }

        return write!(f, "}}");
    }
}

impl syn::parse::Parse for Parameter {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        if !stream.peek(syn::token::Brace) {
            return Err(stream.error(""));
        }

        let mut param = Parameter {
            name: None,
            format: None,
        };

        let content;
        syn::braced!(content in stream);

        if let Ok(ident) = content.parse::<syn::Ident>() {
            param.name = Some(ident.to_string());
        }

        if let Ok(_) = content.parse::<syn::Token![:]>() {
            param.format = Some(content.to_string());
        }

        return Ok(param);
    }
}
