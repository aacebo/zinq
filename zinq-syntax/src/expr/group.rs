use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{LParen, RParen, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Group Expression
/// `(...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupExpr {
    pub span: Span,
    pub left_paren: LParen,
    pub exprs: Vec<Expr>,
    pub right_paren: RParen,
}

impl From<GroupExpr> for Expr {
    fn from(value: GroupExpr) -> Self {
        Self::Group(value)
    }
}

impl Node for GroupExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Group"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for GroupExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for GroupExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for GroupExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let mut exprs = vec![];

        while !cursor.eof() {
            if parser.peek_as::<RParen>(cursor).unwrap_or(true) {
                break;
            }

            exprs.push(parser.parse_as::<Expr>(cursor)?);
        }

        let right_paren = parser.parse_as::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_paren.span(), right_paren.span()),
            left_paren,
            exprs,
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

    use crate::{TokenParser, expr::GroupExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"(test 0.5 b'h')").cursor();
        let value = parser.parse_as::<GroupExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "(test 0.5 b'h')");

        Ok(())
    }
}
