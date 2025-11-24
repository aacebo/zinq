use quote::quote;
use zinq_parse::{
    Error,
    element::{Context, Element, StructContext},
};

pub struct DebugElement;

impl DebugElement {
    pub fn new() -> Self {
        return Self;
    }
}

impl Element for DebugElement {
    type Context = StructContext<proc_macro2::TokenStream>;

    fn select(&self, context: &mut Self::Context) -> bool {
        return context.value().ident.to_string() == "Hello";
    }

    fn render(&self, context: &mut Self::Context) -> Result<proc_macro2::TokenStream, Error> {
        let item = context.value();
        let ident = &context.value().ident;

        Ok(quote! {
            #item

            impl #ident {
                pub fn print_debug_custom(&self) {
                    println!("{:#?}", self);
                }
            }
        })
    }
}

#[test]
pub fn should_debug_value() {
    let item: syn::ItemStruct = syn::parse_quote! {
        #[derive(Debug, Clone)]
        pub struct Hello {
            pub world: bool,
        }
    };

    let mut context = StructContext::new(item.clone()).build();
    let element = DebugElement::new();

    assert!(element.select(&mut context));

    match element.render(&mut context) {
        Err(err) => panic!("{}", err),
        Ok(tokens) => {
            debug_assert_eq!(
                tokens.to_string(),
                quote! {
                    #item

                    impl Hello {
                        pub fn print_debug_custom(&self) {
                            println!("{:#?}", self);
                        }
                    }
                }
                .to_string()
            )
        }
    }
}
