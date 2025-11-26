use syn::parse::Parse;

use crate::Context;

#[derive(Debug, Clone)]
pub struct StructContext<Input: Parse> {
    input: Option<Input>,
    target: syn::ItemStruct,
}

impl<Input: Parse> Context for StructContext<Input> {
    type Input = Input;
    type Target = syn::ItemStruct;

    fn input(&self) -> Option<&Self::Input> {
        return match &self.input {
            None => None,
            Some(input) => Some(input),
        };
    }

    fn target(&self) -> &Self::Target {
        return &self.target;
    }

    fn target_mut(&mut self) -> &mut Self::Target {
        return &mut self.target;
    }
}
