use zinq_parse::Parser;

macro_rules! define_puncts {
    ($($token:literal, pub struct $name:ident, $is_method:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Punct {
            $($name($name),)*
        }

        impl Punct {
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

        impl zinq_parse::Peek<$crate::TokenParser> for Punct {
            #[inline]
            fn peek(cursor: &zinq_parse::Cursor, parser: &$crate::TokenParser) -> zinq_error::Result<bool> {
                $(
                    if let Ok(ok) = parser.peek_as::<$name>(cursor) && ok == true {
                        return Ok(true);
                    }
                )*

                Ok(false)
            }
        }

        impl zinq_parse::Parse<$crate::TokenParser> for Punct {
            #[inline]
            fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut $crate::TokenParser) -> zinq_error::Result<$crate::Token> {
                $(
                    if let Ok(ok) = parser.peek_as::<$name>(cursor) && ok {
                        return parser.parse_as::<$name>(cursor);
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

        impl From<Punct> for $crate::Token {
            #[inline]
            fn from(value: Punct) -> Self {
                Self::Punct(value)
            }
        }

        impl std::fmt::Display for Punct {
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

            impl $name {
                pub fn name(&self) -> &'static str {
                    stringify!($name)
                }
            }

            impl zinq_parse::Peek<$crate::TokenParser> for $name {
                #[inline]
                fn peek(cursor: &zinq_parse::Cursor, _: &$crate::TokenParser) -> zinq_error::Result<bool> {
                    Ok(cursor.peek_n($token.len())? == $token.as_bytes())
                }
            }

            impl zinq_parse::Parse<$crate::TokenParser> for $name {
                #[inline]
                fn parse(cursor: &mut zinq_parse::Cursor, _: &mut $crate::TokenParser) -> zinq_error::Result<$crate::Token> {
                    if !(cursor.next_n($token.len())?.span() == &$token.as_bytes()) {
                        return Err(cursor.error(zinq_error::NOT_FOUND, &format!("expected '{}'", $token)));
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

            impl From<$name> for Punct {
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
                    Self::Punct(Punct::$name(value))
                }
            }

            impl $crate::Token {
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::Punct(punct) => punct.$is_method(),
                        _ => false,
                    }
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

                    debug_assert!(token.is_punct());
                    debug_assert!(token.$is_method());
                    debug_assert_eq!(token.to_string(), $token);

                    Ok(())
                }
            )*
        }
    };
}

define_puncts! {
    "&&",       pub struct AndAnd,       is_and_and,
    "&",        pub struct And,          is_and,
    "@",        pub struct At,           is_at,
    "^=",       pub struct CaretEq,      is_caret_eq,
    "^",        pub struct Caret,        is_caret,
    "::",       pub struct ColonColon,   is_colon_colon,
    ":",        pub struct Colon,        is_colon,
    ";",        pub struct SemiColon,    is_semi_colon,
    ",",        pub struct Comma,        is_comma,
    "$",        pub struct Dollar,       is_dollar,
    "..",       pub struct DotDot,       is_dot_dot,
    ".",        pub struct Dot,          is_dot,
    "==",       pub struct EqEq,         is_eq_eq,
    "=>",       pub struct EqArrow,      is_eq_arrow,
    "=",        pub struct Eq,           is_eq,
    ">=",       pub struct GtEq,         is_gt_eq,
    ">",        pub struct Gt,           is_gt,
    "<=",       pub struct LtEq,         is_lt_eq,
    "<",        pub struct Lt,           is_lt,
    "->",       pub struct RArrow,       is_right_arrow,
    "-=",       pub struct MinusEq,      is_minus_eq,
    "-",        pub struct Minus,        is_minus,
    "!=",       pub struct NotEq,        is_not_eq,
    "!",        pub struct Not,          is_not,
    "||",       pub struct OrOr,         is_or_or,
    "|",        pub struct Or,           is_or,
    "%",        pub struct Percent,      is_percent,
    "+=",       pub struct PlusEq,       is_plus_eq,
    "+",        pub struct Plus,         is_plus,
    "#",        pub struct Pound,        is_pound,
    "?",        pub struct Question,     is_question,
    "/=",       pub struct SlashEq,      is_slash_eq,
    "/",        pub struct Slash,        is_slash,
    "*=",       pub struct StarEq,       is_star_eq,
    "*",        pub struct Star,         is_star,
    "~",        pub struct Tilde,        is_tilde
}
