use crate::LazyParse;

#[derive(Debug, Clone)]
pub struct StructContext<Args: Clone + syn::parse::Parse + quote::ToTokens> {
    args: Option<LazyParse<Args>>,
    value: syn::ItemStruct,
}

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> StructContext<Args> {
    pub fn new(value: syn::ItemStruct) -> StructContextBuilder<Args> {
        return StructContextBuilder::new(value);
    }
}

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> super::Context for StructContext<Args> {
    type Args = Args;
    type Value = syn::ItemStruct;

    fn args(&self) -> Result<Self::Args, crate::Error> {
        match &self.args {
            None => Err(crate::Error::new(
                "attempted to access struct attribute arguments in invalid scope",
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
pub struct StructContextBuilder<Args: Clone + syn::parse::Parse + quote::ToTokens>(
    StructContext<Args>,
);

impl<Args: Clone + syn::parse::Parse + quote::ToTokens> StructContextBuilder<Args> {
    pub fn new(value: syn::ItemStruct) -> Self {
        return Self(StructContext { args: None, value });
    }

    pub fn with_args(&self, args: Args) -> Self {
        let mut next = self.clone();
        next.0.args = Some(LazyParse::from(args));
        return next;
    }

    pub fn build(&self) -> StructContext<Args> {
        return self.0.clone();
    }
}
