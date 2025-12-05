use quote::ToTokens;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Id(Vec<String>);

impl Id {
    pub fn new() -> Self {
        return Self(vec![]);
    }
}

impl From<syn::Path> for Id {
    fn from(value: syn::Path) -> Self {
        return Self(
            value
                .segments
                .iter()
                .map(|s| s.to_token_stream().to_string())
                .collect(),
        );
    }
}

impl From<syn::TypePath> for Id {
    fn from(value: syn::TypePath) -> Self {
        return Self::from(value.path);
    }
}

impl From<syn::Ident> for Id {
    fn from(value: syn::Ident) -> Self {
        return Self(vec![value.to_string()]);
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.0 {
            write!(f, "{}", part)?;
        }

        Ok(())
    }
}

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        return self.to_string() == *other;
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        return self.to_string() == other.to_string();
    }
}
