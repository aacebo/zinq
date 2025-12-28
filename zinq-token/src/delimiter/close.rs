use zinq_parse::Parser;

macro_rules! define_close_delimiters {
    ($($token:literal, pub struct $name:ident, $is_method:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum CloseDelim {
            $($name($name),)*
        }

        impl CloseDelim {
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

        impl zinq_parse::Peek<$crate::TokenParser> for CloseDelim {
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

        impl zinq_parse::Parse<$crate::TokenParser> for CloseDelim {
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

        impl From<CloseDelim> for $crate::Delim {
            #[inline]
            fn from(value: CloseDelim) -> Self {
                Self::Close(value.into())
            }
        }

        impl From<CloseDelim> for $crate::Token {
            #[inline]
            fn from(value: CloseDelim) -> Self {
                Self::Delim(value.into())
            }
        }

        impl std::fmt::Display for CloseDelim {
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

            impl From<$name> for CloseDelim {
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

            impl From<$name> for $crate::Delim {
                #[inline]
                fn from(value: $name) -> Self {
                    Self::Close(CloseDelim::$name(value))
                }
            }

            impl From<$name> for $crate::Token {
                #[inline]
                fn from(value: $name) -> Self {
                    Self::Delim($crate::Delim::Close(CloseDelim::$name(value)))
                }
            }

            impl $crate::Delim {
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::Close(delim) => delim.$is_method(),
                        _ => false,
                    }
                }
            }

            impl $crate::Token {
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::Delim(delim) => delim.$is_method(),
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

                    debug_assert!(token.is_delim());
                    debug_assert!(token.$is_method());
                    debug_assert_eq!(token.to_string(), $token);

                    Ok(())
                }
            )*
        }
    };
}

define_close_delimiters! {
    "}",    pub struct RBrace,      is_right_brace,
    "]",    pub struct RBracket,    is_right_bracket,
    ")",    pub struct RParen,      is_right_paren
}
