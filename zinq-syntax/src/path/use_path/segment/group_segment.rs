use zinq_parse::{Parse, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen};

use crate::path::{UsePath, UseSegment};

///
/// ## Use Group
/// a comma delimited list of
/// sub paths
/// ### Example
/// `use std::string::{String, ToString};`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseGroup {
    pub span: Span,
    pub left_paren: LParen,
    pub items: Punctuated<UsePath, Comma>,
    pub right_paren: RParen,
}

impl From<UseGroup> for UseSegment {
    fn from(value: UseGroup) -> Self {
        Self::Group(value)
    }
}

impl std::fmt::Display for UseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for UseGroup {
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

impl Parse for UseGroup {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let items = parser.parse::<Punctuated<UsePath, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;
        let span = Span::from_bounds(left_paren.span(), right_paren.span());

        Ok(Self {
            span,
            left_paren,
            items,
            right_paren,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
