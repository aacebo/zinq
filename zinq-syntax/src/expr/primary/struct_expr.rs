use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LBrace, Punctuated, RBrace};

use crate::{Node, Path, expr::Expr, members::MemberValue};

///
/// ## Struct Expression
/// ```
/// Struct { a: 1, b: 2 }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructExpr {
    pub path: Path,
    pub left_brace: LBrace,
    pub members: Punctuated<MemberValue, Comma>,
    pub right_brace: RBrace,
}

impl From<StructExpr> for Expr {
    fn from(value: StructExpr) -> Self {
        Self::Struct(value)
    }
}

impl Node for StructExpr {
    fn name(&self) -> &str {
        "Expr::Primary::Struct"
    }
}

impl std::fmt::Display for StructExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for StructExpr {
    fn span(&self) -> Span {
        Span::join(self.path.span(), self.right_brace.span())
    }
}

impl Peek for StructExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for StructExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Path>(cursor)?;
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let members = parser.parse::<Punctuated<MemberValue, Comma>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            path,
            left_brace,
            members,
            right_brace,
        }
        .into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"User { name, age: 5 }").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "User { name, age: 5 }");
        debug_assert_eq!(value.as_struct().members.len(), 2);
        Ok(())
    }
}
