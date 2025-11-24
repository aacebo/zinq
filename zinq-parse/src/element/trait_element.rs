use crate::LazyParse;

#[derive(Debug, Clone)]
pub struct TraitContext<Args: Clone + syn::parse::Parse + quote::ToTokens> {
    args: Option<LazyParse<Args>>,
    value: syn::ItemTrait,
}

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> TraitContext<Args> {
    pub fn new(value: syn::ItemTrait) -> TraitContextBuilder<Args> {
        return TraitContextBuilder::new(value);
    }
}

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> super::Context for TraitContext<Args> {
    type Args = Args;
    type Value = syn::ItemTrait;

    fn args(&self) -> Result<Self::Args, crate::Error> {
        match &self.args {
            None => Err(crate::Error::new(
                "attempted to access trait attribute arguments in invalid scope",
            )),
            Some(args) => Ok(args.get()),
        }
    }

    fn value(&self) -> &Self::Value {
        return &self.value;
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        return &mut self.value;
    }
}

#[derive(Clone)]
pub struct TraitContextBuilder<Args: Clone + syn::parse::Parse + quote::ToTokens>(
    TraitContext<Args>,
);

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> TraitContextBuilder<Args> {
    pub fn new(value: syn::ItemTrait) -> Self {
        return Self(TraitContext { args: None, value });
    }

    pub fn with_args(&self, args: Args) -> Self {
        let mut next = self.clone();
        next.0.args = Some(LazyParse::from(args));
        return next;
    }

    pub fn build(&self) -> TraitContext<Args> {
        return self.0.clone();
    }
}
