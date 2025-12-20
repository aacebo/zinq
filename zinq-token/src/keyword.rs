use zinq_parse::{Parse, Peek};

macro_rules! define_keywords {
    ($($token:literal pub struct $name:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl zinq_parse::Peek for Keyword {
            #[inline]
            fn peek<P: zinq_parse::Parser>(parser: &P) -> bool {
                $(
                    if parser.peek_as::<$name>() {
                        return true;
                    }
                )*

                false
            }
        }

        impl zinq_parse::Parse for Keyword {
            #[inline]
            fn parse<P: zinq_parse::Parser>(parser: &mut P) -> zinq_error::Result<Self> {
                $(
                    if parser.peek_as::<$name>() {
                        if let Some(v) = parser.parse::<$name>()?.value() {
                            return Ok(v.value().clone().into());
                        }
                    }
                )*

                Err(parser.error(&format!("unknown tokens '{}'", parser.span())))
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

            impl $name {
                #[inline]
                pub fn span(&self) -> &zinq_parse::Span {
                    &self.span
                }
            }

            impl zinq_parse::Peek for $name {
                #[inline]
                fn peek<P: zinq_parse::Parser>(parser: &P) -> bool {
                    parser.span().matches($token.as_bytes())
                }
            }

            impl zinq_parse::Parse for $name {
                #[inline]
                fn parse<P: zinq_parse::Parser>(parser: &mut P) -> zinq_error::Result<Self> {
                    if !parser.span().matches($token.as_bytes()) {
                        return Err(parser.error(&format!("expected '{}'", $token)));
                    }

                    Ok(Self {
                        span: parser.span().clone(),
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
