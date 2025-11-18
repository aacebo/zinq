use std::sync::Arc;

#[derive(Clone)]
pub struct Arguments(Vec<Argument>);

impl Arguments {
    pub fn new() -> ArgumentsBuilder {
        return ArgumentsBuilder::new();
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

impl std::ops::Index<usize> for Arguments {
    type Output = Argument;

    fn index(&self, index: usize) -> &Self::Output {
        return self.0.index(index);
    }
}

impl std::ops::Index<&str> for Arguments {
    type Output = Argument;

    fn index(&self, index: &str) -> &Self::Output {
        return self
            .0
            .iter()
            .find(|arg| {
                if let Some(name) = &arg.name {
                    return name == index;
                }

                false
            })
            .expect("argument not found");
    }
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (i, arg) in self.0.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i < self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }

        return write!(f, "}}");
    }
}

#[derive(Clone)]
pub struct ArgumentsBuilder(Vec<Argument>);

impl ArgumentsBuilder {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn with<T: std::fmt::Display + std::any::Any>(&self, value: T) -> Self {
        let mut next = self.clone();

        next.0.push(Argument {
            name: None,
            value: Arc::new(value),
        });

        return next;
    }

    pub fn with_named<T: std::fmt::Display + std::any::Any>(&self, key: &str, value: T) -> Self {
        let mut next = self.clone();

        next.0.push(Argument {
            name: Some(key.to_string()),
            value: Arc::new(value),
        });

        return next;
    }

    pub fn build(&self) -> Arguments {
        return Arguments(self.0.clone());
    }
}

#[derive(Clone)]
pub struct Argument {
    name: Option<String>,
    value: Arc<dyn std::fmt::Display>,
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.value);
    }
}
