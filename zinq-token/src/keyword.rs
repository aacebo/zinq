use zinq_parse::{Parse, Parser, Peek};

macro_rules! define_keywords {
    ($($token:literal, pub struct $name:ident, $is_method:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl Keyword {
            #[inline]
            pub fn try_from_span(span: &zinq_parse::Span) -> Option<Self> {
                $(
                    if $token.as_bytes() == span.bytes() {
                        return Some(Self::$name($name {
                            span: span.clone(),
                        }));
                    }
                )*

                None
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
            fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<$crate::Token> {
                $(
                    if parser.peek_as::<$name>(cursor) {
                        return Ok(parser.parse_as::<$name>(cursor)?.clone().into());
                    }
                )*

                Err(cursor.error(&format!("unknown tokens '{}'", cursor.span())).build().into())
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
                fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<$crate::Token> {
                    if !(cursor.span() == &$token.as_bytes()) {
                        return Err(cursor.error(&format!("expected '{}'", $token)).build().into());
                    }

                    Ok(Self {
                        span: cursor.span().clone(),
                    }.into())
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
        )*
    };
}

define_keywords! {
    "mod",       pub struct Mod,        is_mod,
    "mut",       pub struct Mut,        is_mut,
    "match",     pub struct Match,      is_match,
    "where",     pub struct Where,      is_where,
    "continue",  pub struct Continue,   is_continue,
    "trait",     pub struct Trait,      is_trait,
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
    "pub",       pub struct Pub,        is_pub,
    "use",       pub struct Use,        is_use
}

#[cfg(test)]
mod test {
    use zinq_parse::Span;

    use crate::Keyword;

    #[test]
    fn should_parse_mod() {
        let span = Span::from_str("mod");
        let mut token = Keyword::try_from_span(&span).expect("should have keyword");

        debug_assert!(token.is_mod());
        debug_assert_eq!(token.to_string(), "mod");
    }

    #[test]
    fn should_parse_pub() {
        let span = Span::from_str("pub");
        let mut token = Keyword::try_from_span(&span).expect("should have keyword");

        debug_assert!(token.is_pub());
        debug_assert_eq!(token.to_string(), "pub");
    }
}
