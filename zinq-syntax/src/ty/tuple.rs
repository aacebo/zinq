use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen, TokenParser};

use crate::{Node, ty::Type};

///
/// ## Tuple Type
/// `(u32, string, bool)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType {
    pub span: Span,
    pub left_paren: LParen,
    pub items: Punctuated<Type, Comma>,
    pub right_paren: RParen,
}

impl From<TupleType> for Type {
    fn from(value: TupleType) -> Self {
        Self::Tuple(value)
    }
}

impl Node for TupleType {
    fn name(&self) -> &str {
        "Syntax::Type::Tuple"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for TupleType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for TupleType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let items = parser.parse_as::<Punctuated<Type, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_paren.span(), right_paren.span()),
            left_paren,
            items,
            right_paren,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{TokenParser, ty::TupleType};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"(u32, string, &mut hello::World)").cursor();
        let value = parser.parse_as::<TupleType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "(u32, string, &mut hello::World)");
        debug_assert_eq!(value.items.len(), 3);
        debug_assert_eq!(
            value.items.get(2).unwrap().0.to_string(),
            "&mut hello::World"
        );

        Ok(())
    }
}
