use zinq_parse::Parser;

macro_rules! define_keywords {
    ($($token:literal, pub struct $name:ident, $is_method:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl Keyword {
            #[inline]
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$name(v) => v.name(),)*
                }
            }

            $(
                #[inline]
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::$name(_) => true,
                        _ => false,
                    }
                }
            )*
        }

        impl zinq_parse::Peek<$crate::TokenParser> for Keyword {
            #[inline]
            fn peek(cursor: &zinq_parse::Cursor, parser: &$crate::TokenParser) -> zinq_error::Result<bool> {
                $(
                    if let Ok(ok) = parser.peek_as::<$name>(cursor) && ok {
                        return Ok(true);
                    }
                )*

                Ok(false)
            }
        }

        impl zinq_parse::Parse<$crate::TokenParser> for Keyword {
            #[inline]
            fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<Self> {
                $(
                    if let Ok(ok) = parser.peek_as::<$name>(cursor) && ok {
                        return Ok(parser.parse_as::<$name>(cursor)?.into());
                    }
                )*

                Err(cursor.error(zinq_error::NOT_FOUND, &format!("unknown tokens '{}'", cursor)))
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

        impl std::fmt::Display for Keyword {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                }
            }
        }

        impl $crate::ToTokens for Keyword {
            fn to_tokens(&self) -> zinq_error::Result<$crate::TokenStream> {
                Ok($crate::Token::Keyword(self.clone()).into())
            }
        }

        $(
            #[doc = concat!('`', $token, '`')]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                span: zinq_parse::Span,
            }

            impl $name {
                pub fn name(&self) -> &'static str {
                    stringify!($name)
                }
            }

            impl zinq_parse::Peek<$crate::TokenParser> for $name {
                #[inline]
                fn peek(cursor: &zinq_parse::Cursor, _: &$crate::TokenParser) -> zinq_error::Result<bool> {
                    Ok(cursor.fork().next_while(|b, _| b.is_ascii_alphanumeric())?.span() == &$token.as_bytes())
                }
            }

            impl zinq_parse::Parse<$crate::TokenParser> for $name {
                #[inline]
                fn parse(cursor: &mut zinq_parse::Cursor, _: &mut $crate::TokenParser) -> zinq_error::Result<Self> {
                    let span = cursor.next_n($token.len())?.span();

                    if span != &$token.as_bytes() {
                        return Err(cursor.error(zinq_error::NOT_FOUND, &format!("expected '{}'", $token)));
                    }

                    Ok(Self {
                        span: span.clone(),
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

            impl std::fmt::Display for $name {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", $token)
                }
            }

            impl From<$name> for $crate::Token {
                #[inline]
                fn from(value: $name) -> Self {
                    Self::Keyword(Keyword::$name(value))
                }
            }

            impl $crate::Token {
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::Keyword(keyword) => keyword.$is_method(),
                        _ => false,
                    }
                }
            }

            impl $crate::ToTokens for $name {
                fn to_tokens(&self) -> zinq_error::Result<$crate::TokenStream> {
                    Ok($crate::Token::Keyword($crate::Keyword::$name(self.clone())).into())
                }
            }
        )*

        #[cfg(test)]
        mod test {
            use zinq_error::Result;
            use zinq_parse::{Parser, Span};

            use crate::TokenParser;

            $(
                #[test]
                fn $is_method() -> Result<()> {
                    let mut cursor = Span::from_str($token).cursor();
                    let mut parser = TokenParser;
                    let token = parser.parse(&mut cursor)?;

                    debug_assert!(token.is_keyword());
                    debug_assert!(token.$is_method());
                    debug_assert_eq!(token.to_string(), $token);

                    Ok(())
                }
            )*
        }
    };
}

define_keywords! {
    "crate",     pub struct Crate,      is_crate,
    "mod",       pub struct Mod,        is_mod,
    "mut",       pub struct Mut,        is_mut,
    "match",     pub struct Match,      is_match,
    "where",     pub struct Where,      is_where,
    "continue",  pub struct Continue,   is_continue,
    "iface",     pub struct IFace,      is_iface,
    "if",        pub struct If,         is_if,
    "else",      pub struct Else,       is_else,
    "for",       pub struct For,        is_for,
    "in",        pub struct In,         is_in,
    "as",        pub struct As,         is_as,
    "let",       pub struct Let,        is_let,
    "const",     pub struct Const,      is_const,
    "enum",      pub struct Enum,       is_enum,
    "impl",      pub struct Impl,       is_impl,
    "fn",        pub struct Fn,         is_fn,
    "return",    pub struct Return,     is_return,
    "struct",    pub struct Struct,     is_struct,
    "self",      pub struct SelfValue,  is_self_type,
    "Self",      pub struct SelfType,   is_self_value,
    "super",     pub struct Super,      is_super,
    "pub",       pub struct Pub,        is_pub,
    "use",       pub struct Use,        is_use
}
