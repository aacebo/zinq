#![allow(unused)]

mod group;
mod ident;
mod keyword;
mod literal;
mod punct;

pub use group::*;
pub use ident::*;
pub use keyword::*;
pub use literal::*;
pub use punct::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek};

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
}

impl Peek for Token {
    #[inline]
    fn peek<P: zinq_parse::Parser>(parser: &P) -> bool {
        Keyword::peek(parser)
    }
}

impl Parse for Token {
    #[inline]
    fn parse<P: zinq_parse::Parser>(parser: &mut P) -> Result<Self> {
        match Keyword::parse(parser) {
            Err(err) => Err(err),
            Ok(v) => Ok(v.into()),
        }
    }

    #[inline]
    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Keyword(v) => v.span(),
        }
    }
}
