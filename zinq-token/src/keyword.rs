use zinq_parse::{Parse, Parser, Peek};

macro_rules! define_keywords {
    ($($token:literal pub struct $name:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl zinq_parse::Peek<$crate::TokenParser> for Keyword {
            #[inline]
            fn peek(cursor: &zinq_parse::Cursor, parser: &$crate::TokenParser) -> bool {
                $(
                    if parser.peek_as::<$name>(cursor) {
                        return true;
                    }
                )*

                false
            }
        }

        impl zinq_parse::Parse<$crate::TokenParser> for Keyword {
            #[inline]
            fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<Self> {
                $(
                    if parser.peek_as::<$name>(cursor) {
                        return Ok(parser.parse_as::<$name>(cursor)?.last().clone().into());
                    }
                )*

                Err(cursor.error(&format!("unknown tokens '{}'", cursor.span())))
            }

            #[inline]
            fn span(&self) -> &zinq_parse::Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }
        }

        impl From<Keyword> for $crate::Token {
            #[inline]
            fn from(value: Keyword) -> Self {
                Self::Keyword(value)
            }
        }

        $(
            #[doc = concat!('`', $token, '`')]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                span: zinq_parse::Span,
            }

            impl zinq_parse::Peek<$crate::TokenParser> for $name {
                #[inline]
                fn peek(cursor: &zinq_parse::Cursor, parser: &$crate::TokenParser) -> bool {
                    cursor.span() == &$token.as_bytes()
                }
            }

            impl zinq_parse::Parse<$crate::TokenParser> for $name {
                #[inline]
                fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<Self> {
                    if !(cursor.span() == &$token.as_bytes()) {
                        return Err(cursor.error(&format!("expected '{}'", $token)));
                    }

                    Ok(Self {
                        span: cursor.span().clone(),
                    })
                }

                #[inline]
                fn span(&self) -> &zinq_parse::Span {
                    &self.span
                }
            }

            impl From<$name> for Keyword {
                #[inline]
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )*
    };
}

define_keywords! {
    "mod"       pub struct Mod,
    "mut"       pub struct Mut,
    "match"     pub struct Match,
    "where"     pub struct Where,
    "continue"  pub struct Continue,
    "trait"     pub struct Trait,
    "if"        pub struct If,
    "else"      pub struct Else,
    "for"       pub struct For,
    "in"        pub struct In,
    "as"        pub struct As,
    "let"       pub struct Let,
    "const"     pub struct Const,
    "enum"      pub struct Enum,
    "impl"      pub struct Impl,
    "fn"        pub struct Fn,
    "return"    pub struct Return,
    "struct"    pub struct Struct,
    "self"      pub struct SelfValue,
    "Self"      pub struct SelfType,
    "pub"       pub struct Pub,
    "use"       pub struct Use
}
