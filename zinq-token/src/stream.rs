use std::{ops::Index, str::FromStr};

use zinq_error::ZinqError;
use zinq_parse::{Parse, Peek, Span};

use crate::{ToTokens, Token};

#[derive(Debug, Default, Clone)]
pub struct TokenStream {
    span: Span,
    inner: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            span: Span::default(),
            inner: vec![],
        }
    }
}

impl TryFrom<&[u8]> for TokenStream {
    type Error = ZinqError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Self::new());
        }

        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(value).cursor();

        parser.parse::<Self>(&mut cursor)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for TokenStream {
    type Error = ZinqError;

    fn try_from(value: &[u8; N]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Self::new());
        }

        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(value).cursor();

        parser.parse::<Self>(&mut cursor)
    }
}

impl FromStr for TokenStream {
    type Err = ZinqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::new());
        }

        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_str(s).cursor();

        parser.parse::<Self>(&mut cursor)
    }
}

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        Self {
            span: value.span().clone(),
            inner: vec![value],
        }
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        let tokens = iter.into_iter().collect::<Vec<Token>>();

        Self {
            span: Span::from_bounds(
                tokens.first().unwrap().span(),
                tokens.last().unwrap().span(),
            ),
            inner: tokens,
        }
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        let tokens = iter
            .into_iter()
            .flat_map(|v| v.inner)
            .collect::<Vec<Token>>();

        Self {
            span: Span::from_bounds(
                tokens.first().unwrap().span(),
                tokens.last().unwrap().span(),
            ),
            inner: tokens,
        }
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl Extend<Self> for TokenStream {
    fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
        self.inner.extend(iter.into_iter().flat_map(|v| v.inner))
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
        &self.inner
    }
}

impl Eq for TokenStream {}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }

        for i in 0..self.inner.len() {
            if self.inner.index(i) != other.inner.index(i) {
                return false;
            }
        }

        true
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(self.clone())
    }
}

impl Peek for TokenStream {
    fn peek(_: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
        Ok(true)
    }
}

impl Parse for TokenStream {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let mut tokens = vec![];

        while !cursor.eof() {
            let token = parser.parse::<Token>(cursor)?;
            tokens.push(token);
        }

        Ok(Self {
            span: Span::from_bounds(
                tokens.first().unwrap().span(),
                tokens.last().unwrap().span(),
            ),
            inner: tokens,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use zinq_error::Result;

    use crate::TokenStream;

    #[test]
    fn should_parse_str() -> Result<()> {
        let stream = TokenStream::from_str("let test: string = \"test\";")?;

        debug_assert_eq!(stream.len(), 7);
        debug_assert_eq!(stream.to_string(), "let test: string = \"test\";");

        Ok(())
    }

    #[test]
    fn should_check_equality() -> Result<()> {
        let a = TokenStream::from_str("let a = b'a'")?;
        let b = TokenStream::try_from(b"let a = b'a'")?;

        debug_assert_eq!(a, b);
        Ok(())
    }
}
