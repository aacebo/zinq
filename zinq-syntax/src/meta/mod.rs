mod attribute;

pub use attribute::*;
use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LBracket, Pound, Punctuated, RBracket, Suffixed};

///
/// ## Meta
///
/// - if feature flag
/// - add `.clone()`
/// - add `.print()` and format pretty
///
/// ```
/// #[If(feature = "...", test), Clone, Print(pretty)]
/// pub struct MyStruct {
///     ...
/// }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    pub pound: Pound,
    pub left_bracket: LBracket,
    pub attrs: Punctuated<Attribute, Comma>,
    pub right_bracket: RBracket,
}

impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for Meta {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.pound.span(), self.right_bracket.span())
    }
}

impl Peek for Meta {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser
            .peek::<Suffixed<Pound, LBracket>>(cursor)
            .unwrap_or(false))
    }
}

impl Parse for Meta {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let pound = parser.parse::<Pound>(cursor)?;
        let left_bracket = parser.parse::<LBracket>(cursor)?;
        let attrs = parser.parse::<Punctuated<Attribute, Comma>>(cursor)?;
        let right_bracket = parser.parse::<RBracket>(cursor)?;

        Ok(Self {
            pound,
            left_bracket,
            attrs,
            right_bracket,
        })
    }
}

#[cfg(test)]
mod test {
    use std::ops::Index;

    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::meta::Meta;

    #[test]
    fn should_parse_one() -> Result<()> {
        let mut cursor = Span::from_bytes(b"#[Clone]").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let meta = parser.parse::<Meta>(&mut cursor)?;

        debug_assert_eq!(meta.attrs.len(), 1);
        debug_assert_eq!(meta.to_string(), "#[Clone]");

        Ok(())
    }

    #[test]
    fn should_parse_with_one_argument() -> Result<()> {
        let mut cursor = Span::from_bytes(b"#[Print(pretty)]").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let meta = parser.parse::<Meta>(&mut cursor)?;

        debug_assert_eq!(meta.attrs.len(), 1);
        debug_assert_eq!(meta.to_string(), "#[Print(pretty)]");
        debug_assert_eq!(meta.attrs.index(0).to_string(), "Print(pretty)");
        debug_assert_eq!(
            meta.attrs
                .index(0)
                .value()
                .args
                .as_ref()
                .unwrap()
                .to_string(),
            "(pretty)"
        );

        Ok(())
    }
}
