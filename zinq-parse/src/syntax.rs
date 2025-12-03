use std::sync::Arc;

use proc_macro2::TokenStream;
use quote::quote;

use crate::Element;

#[derive(Clone)]
pub struct Syntax {
    elements: Vec<Arc<dyn Element>>,
}

impl Syntax {
    pub fn new() -> Self {
        return Self { elements: vec![] };
    }

    pub fn with_element<T: Element + 'static>(&mut self, element: T) -> &mut Self {
        self.elements.push(Arc::new(element));
        return self;
    }
}

impl Syntax {
    pub fn parse(&self, input: Option<TokenStream>, tokens: TokenStream) -> TokenStream {
        let mut res = quote!();

        for element in &self.elements {
            let mut context = match element.parse(input.clone(), tokens.clone()) {
                Err(err) => return err.to_compile_error(),
                Ok(v) => v,
            };

            let rendered = match element.render(&mut context) {
                Err(err) => err.to_compile_error(),
                Ok(v) => v,
            };

            res.extend(rendered);
        }

        return res;
    }
}
