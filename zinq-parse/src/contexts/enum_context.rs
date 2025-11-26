use syn::parse::Parse;

use crate::Context;

#[derive(Debug, Clone)]
pub struct EnumContext<Input: Parse> {
    input: Option<Input>,
    target: syn::ItemEnum,
}

impl<Input: Parse> Context for EnumContext<Input> {
    type Input = Input;
    type Target = syn::ItemEnum;

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
