use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Enclosed, LParen, RParen, Token, TokenStream};

use crate::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub path: Path,
    pub args: Option<Enclosed<LParen, TokenStream, RParen>>,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for Attribute {
    fn span(&self) -> zinq_parse::Span {
        if let Some(args) = &self.args {
            return Span::join(self.path.span(), args.span());
        }

        self.path.span()
    }
}

impl Peek for Attribute {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Path>(cursor).unwrap_or(false))
    }
}

impl Parse for Attribute {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Path>(cursor)?;

        if parser.peek::<LParen>(cursor).unwrap_or(false) {
            let mut args = TokenStream::new();
            let start = parser.parse::<LParen>(cursor)?;

            while !parser.peek::<RParen>(cursor)? {
                let token = parser.parse::<Token>(cursor)?;
                args.push(&token)?;
            }

            let end = parser.parse::<RParen>(cursor)?;

            return Ok(Self {
                path,
                args: Some(Enclosed {
                    start,
                    end,
                    inner: args,
                }),
            });
        }

        Ok(Self { path, args: None })
    }
}
