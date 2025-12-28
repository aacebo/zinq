use std::{ops::Index, str::FromStr};

use zinq_error::ZinqError;
use zinq_parse::{Parser, Span};

use crate::{Token, TokenParser};

#[derive(Debug, Default, Clone)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl TryFrom<&[u8]> for TokenStream {
    type Error = ZinqError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut parser = TokenParser;
        let span = Span::from_bytes(value);
        let mut cursor = span.cursor();
        let mut tokens = vec![];

        while !cursor.eof() {
            let token = parser.parse(&mut cursor)?;
            tokens.push(token);
        }

        Ok(Self(tokens))
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for TokenStream {
    type Error = ZinqError;

    fn try_from(value: &[u8; N]) -> Result<Self, Self::Error> {
        let mut parser = TokenParser;
        let span = Span::from_bytes(value);
        let mut cursor = span.cursor();
        let mut tokens = vec![];

        while !cursor.eof() {
            let token = parser.parse(&mut cursor)?;
            tokens.push(token);
        }

        Ok(Self(tokens))
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

        Ok(Self(tokens))
    }
}

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        Self(vec![value])
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Vec<Token>>())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self(iter.into_iter().flat_map(|v| v.0).collect::<Vec<Token>>())
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl Extend<Self> for TokenStream {
    fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().flat_map(|v| v.0))
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for TokenStream {}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for i in 0..self.0.len() {
            if self.0.index(i) != other.0.index(i) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use zinq_error::Result;

    use crate::TokenStream;

    #[test]
    fn should_parse_str() -> Result<()> {
        let mut stream = TokenStream::from_str("let test: string = \"test\";")?;

        debug_assert_eq!(stream.len(), 7);
        debug_assert_eq!(stream.to_string(), "lettest:string=\"test\";");

        stream.extend(TokenStream::from_str("\nprintln(\"hello world\");")?);

        debug_assert_eq!(stream.len(), 12);
        debug_assert_eq!(
            stream.to_string(),
            "lettest:string=\"test\";println(\"hello world\");"
        );

        Ok(())
    }

    #[test]
    fn should_check_equality() -> Result<()> {
        let a = TokenStream::from_str("\n\nlet a = b'a'")?;
        let b = TokenStream::try_from(b"let a = b'a'")?;

        debug_assert_eq!(a.to_string(), b.to_string());
        Ok(())
    }
}
