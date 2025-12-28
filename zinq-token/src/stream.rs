use std::{ops::Index, str::FromStr};

use zinq_error::ZinqError;
use zinq_parse::{Parse, Parser, Span};

use crate::{Token, TokenParser};

#[derive(Debug, Default, Clone)]
pub struct TokenStream {
    span: Span,
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            span: Span::from_str(""),
            tokens: vec![],
        }
    }
}

impl FromStr for TokenStream {
    type Err = ZinqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = TokenParser;
        let span = Span::from_str(s);
        let mut cursor = span.cursor();
        let mut tokens = vec![];

        while !cursor.eof() {
            let token = parser.parse(&mut cursor)?;
            tokens.push(token);
        }

        Ok(Self {
            span: span.clone(),
            tokens,
        })
    }
}

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        Self {
            span: value.span().clone(),
            tokens: vec![value],
        }
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        let tokens = iter.into_iter().collect::<Vec<Token>>();

        if !tokens.is_empty() {
            return Self {
                span: Span::from_bounds(
                    tokens.first().expect("must not be empty").span(),
                    tokens.last().expect("must not be empty").span(),
                ),
                tokens,
            };
        }

        Self::default()
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        let tokens = iter
            .into_iter()
            .flat_map(|v| v.tokens)
            .collect::<Vec<Token>>();

        if !tokens.is_empty() {
            return Self {
                span: Span::from_bounds(
                    tokens.first().expect("must not be empty").span(),
                    tokens.last().expect("must not be empty").span(),
                ),
                tokens,
            };
        }

        Self::default()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.tokens.extend(iter);
        self.span = Span::from_bounds(
            self.tokens.first().expect("must not be empty").span(),
            self.tokens.last().expect("must not be empty").span(),
        );
    }
}

impl Extend<Self> for TokenStream {
    fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
        self.tokens.extend(iter.into_iter().flat_map(|v| v.tokens));
        self.span = Span::from_bounds(
            self.tokens.first().expect("must not be empty").span(),
            self.tokens.last().expect("must not be empty").span(),
        );
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl Eq for TokenStream {}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if self.tokens.len() != other.tokens.len() {
            return false;
        }

        for i in 0..self.tokens.len() {
            if self.tokens.index(i) != other.tokens.index(i) {
                return false;
            }
        }

        true
    }
}

// #[cfg(test)]
// mod test {
//     use std::str::FromStr;

//     use zinq_error::Result;

//     use crate::TokenStream;

//     #[test]
//     fn should_parse_str() -> Result<()> {
//         let mut stream = TokenStream::from_str("let test: string = \"test\";")?;

//         debug_assert_eq!(stream.len(), 7);
//         debug_assert_eq!(stream.to_string(), "let test: string = \"test\";");

//         stream.extend(TokenStream::from_str("\nprintln(\"hello world\");")?);

//         debug_assert_eq!(stream.len(), 12);
//         debug_assert_eq!(
//             stream.to_string(),
//             "let test: string = \"test\";\nprintln(\"hello world\");"
//         );

//         Ok(())
//     }
// }
