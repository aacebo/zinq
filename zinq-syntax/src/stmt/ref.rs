use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, TokenParser};

use crate::{Node, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefStmt {
    pub span: Span,
    pub and: And,
    pub to: Box<Stmt>,
}

impl From<RefStmt> for Stmt {
    fn from(value: RefStmt) -> Self {
        Self::Ref(value)
    }
}

impl Node for RefStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Ref"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for RefStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for RefStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for RefStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse_as::<And>(cursor)?;
        let to = parser.parse_as::<Box<Stmt>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(and.span(), to.span()),
            and,
            to,
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

    use crate::{TokenParser, stmt::RefStmt};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&int").cursor();
        let value = parser.parse_as::<RefStmt>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&int");
        debug_assert_eq!(value.to.to_string(), "int");

        Ok(())
    }

    #[test]
    fn should_parse_mut() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&mut int").cursor();
        let value = parser.parse_as::<RefStmt>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&mut int");
        debug_assert_eq!(value.to.to_string(), "mut int");

        Ok(())
    }
}
