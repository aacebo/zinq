#[derive(Debug, Clone)]
pub struct EnumParams {
    pub name: Option<String>,
}

impl syn::parse::Parse for EnumParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(literal) = input.parse::<syn::LitStr>() {
            return Ok(Self {
                name: Some(literal.value()),
            });
        }

        let mut params = Self { name: None };

        while let Ok(key) = input.parse::<syn::Ident>() {
            input.parse::<syn::Token![=]>()?;

            match key.to_string().as_str() {
                "name" => {
                    let value: syn::LitStr = input.parse()?;
                    params.name = Some(value.value());
                }
                _ => {
                    return Err(syn::parse::Error::new(
                        key.span(),
                        format!("error attribute parameter '{}' is invalid", &key),
                    ));
                }
            };
        }

        return Ok(params);
    }
}
