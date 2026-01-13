use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{And, Mut, SelfValue};

use crate::Node;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelfParam {
    pub and: Option<And>,
    pub mutable: Option<Mut>,
    pub keyword: SelfValue,
}

impl std::fmt::Display for SelfParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for SelfParam {
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

impl Parse for SelfParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse::<Option<And>>(cursor)?;
        let mutable = parser.parse::<Option<Mut>>(cursor)?;
        let keyword = parser.parse::<SelfValue>(cursor)?;

        Ok(Self {
            and,
            mutable,
            keyword,
        })
    }
}

impl Spanned for SelfParam {
    fn span(&self) -> Span {
        let mut first = self.keyword.span();

        if let Some(v) = &self.mutable {
            first = v.span().clone();
        }

        if let Some(v) = &self.and {
            first = v.span().clone();
        }

        Span::join(first, self.keyword.span())
    }
}

impl Node for SelfParam {
    fn name(&self) -> &str {
        "Param::Self"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_self_param(self);
    }
}
