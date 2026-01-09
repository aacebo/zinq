use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, Gt, Lt, Punctuated};

use crate::param::TypeParam;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generics {
    pub lt: Lt,
    pub params: Punctuated<TypeParam, Comma>,
    pub gt: Gt,
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for Generics {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.lt.span(), self.gt.span())
    }
}

impl Peek for Generics {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Lt>(cursor).unwrap_or(false))
    }
}

impl Parse for Generics {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let lt = parser.parse::<Lt>(cursor)?;
        let params = parser.parse::<Punctuated<TypeParam, Comma>>(cursor)?;
        let gt = parser.parse::<Gt>(cursor)?;

        Ok(Self { lt, params, gt })
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::Generics;

    #[test]
    fn should_parse_one() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"<T>").cursor();

        let syntax = parser.parse::<Generics>(&mut cursor)?;

        debug_assert_eq!(syntax.to_string(), "<T>");
        debug_assert_eq!(syntax.params.len(), 1);

        Ok(())
    }

    #[test]
    fn should_parse_many() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"<A, B, C>").cursor();

        let syntax = parser.parse::<Generics>(&mut cursor)?;

        debug_assert_eq!(syntax.to_string(), "<A, B, C>");
        debug_assert_eq!(syntax.params.len(), 3);

        Ok(())
    }

    #[test]
    fn should_parse_with_one_bound() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"<T: std::string::ToString>").cursor();

        let syntax = parser.parse::<Generics>(&mut cursor)?;

        debug_assert_eq!(syntax.to_string(), "<T: std::string::ToString>");
        debug_assert_eq!(syntax.params.len(), 1);

        Ok(())
    }

    #[test]
    fn should_parse_with_many_bound() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"<T: std::string::ToString + Cmp, B>").cursor();

        let syntax = parser.parse::<Generics>(&mut cursor)?;

        debug_assert_eq!(syntax.to_string(), "<T: std::string::ToString + Cmp, B>");
        debug_assert_eq!(syntax.params.len(), 2);

        Ok(())
    }
}
