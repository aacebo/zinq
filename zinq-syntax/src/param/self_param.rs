use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, Mut, SelfValue, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfParam {
    pub span: Span,
    pub and: Option<And>,
    pub mutable: Option<Mut>,
    pub keyword: SelfValue,
}

impl std::fmt::Display for SelfParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for SelfParam {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for SelfParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse_as::<Option<And>>(cursor)?;
        let mutable = parser.parse_as::<Option<Mut>>(cursor)?;
        let keyword = parser.parse_as::<SelfValue>(cursor)?;
        let mut first = keyword.span().clone();

        if let Some(v) = &mutable {
            first = v.span().clone();
        }

        if let Some(v) = &and {
            first = v.span().clone();
        }

        let span = Span::from_bounds(&first, keyword.span());

        Ok(Self {
            span,
            and,
            mutable,
            keyword,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
