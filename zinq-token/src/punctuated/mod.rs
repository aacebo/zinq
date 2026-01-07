mod item;

use zinq_parse::{Parse, Peek, Span};

use crate::punctuated::item::Item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    span: Span,
    items: Vec<Item<T, P>>,
}

impl<T, P> std::ops::Deref for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    type Target = [Item<T, P>];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T, P> Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: Item<T, P>) -> &mut Self {
        self.items.push(item);
        self
    }
}

impl<T, P> std::fmt::Display for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<T, P> Peek for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        parser.peek::<T>(cursor)
    }
}

impl<T, P> Parse for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let mut items = vec![];
        let start = cursor.span().clone();

        while parser.peek::<Item<T, P>>(cursor).unwrap_or(false) {
            let item = parser.parse::<Item<T, P>>(cursor)?;

            items.push(item.clone());

            if item.is_final() {
                break;
            }
        }

        let end = cursor.span();

        Ok(Self {
            span: Span::from_bounds(&start, end),
            items,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::{ColonColon, Comma, Ident, LInt, Punctuated, Token};

    #[test]
    fn should_parse_int_list() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"1, 2, 3").cursor();
        let stream = parser.parse::<Punctuated<LInt, Comma>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "1, 2, 3");

        Ok(())
    }

    #[test]
    fn should_parse_ident_list() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"hello::world::mod").cursor();
        let stream = parser.parse::<Punctuated<Token, ColonColon>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "hello::world::mod");

        Ok(())
    }

    #[test]
    fn should_parse_with_trailing() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"hello::world::mod::").cursor();
        let stream = parser.parse::<Punctuated<Ident, ColonColon>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "hello::world::mod::");

        Ok(())
    }
}
