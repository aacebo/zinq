#[derive(Debug, Clone)]
pub struct VariantParams {
    pub name: Option<String>,
    pub code: Option<u16>,
    pub message: Option<String>,
}

impl syn::parse::Parse for VariantParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(literal) = input.parse::<syn::LitStr>() {
            return Ok(Self {
                name: None,
                code: None,
                message: Some(literal.value()),
            });
        }

        let mut params = Self {
            name: None,
            code: None,
            message: None,
        };

        while let Ok(key) = input.parse::<syn::Ident>() {
            input.parse::<syn::Token![=]>()?;

            match key.to_string().as_str() {
                "name" => {
                    let value: syn::LitStr = input.parse()?;
                    params.name = Some(value.value());
                }
                "code" => {
                    let value: syn::LitInt = input.parse()?;
                    params.code = Some(value.base10_parse()?);
                }
                "message" => {
                    let value: syn::LitStr = input.parse()?;
                    params.message = Some(value.value());
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
