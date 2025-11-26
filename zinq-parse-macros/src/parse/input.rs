#![allow(unused)]

use syn::punctuated::Punctuated;

use crate::parse::InputArgument;

#[derive(Debug, Clone)]
pub struct Input {
    context: Option<syn::Type>,
    output: Option<syn::Type>,
}

impl Input {
    pub fn context(&self) -> Option<&syn::Type> {
        return match &self.context {
            None => None,
            Some(v) => Some(v),
        };
    }

    pub fn output(&self) -> Option<&syn::Type> {
        return match &self.output {
            None => None,
            Some(v) => Some(v),
        };
    }
}

impl syn::parse::Parse for Input {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut context: Option<syn::Type> = None;
        let mut output: Option<syn::Type> = None;
        let args =
            Punctuated::<InputArgument<syn::Type>, syn::Token![,]>::parse_terminated(stream)?;

        for arg in args {
            let name = arg.name().to_string();

            match name.as_str() {
                "Context" => {
                    context = Some(arg.value().clone());
                }
                "Output" => {
                    output = Some(arg.value().clone());
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        arg.name(),
                        "unknown argument (expected 'Output or Context')",
                    ));
                }
            };
        }

        Ok(Self { context, output })
    }
}
