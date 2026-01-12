macro_rules! define_open_delimiters {
    ($($token:literal, pub struct $name:ident, $is_method:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum OpenDelim {
            $($name($name),)*
        }

        impl OpenDelim {
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

        impl zinq_parse::Peek for OpenDelim {
            #[inline]
            fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
                $(
                    if let Ok(ok) = parser.peek::<$name>(cursor) && ok == true {
                        return Ok(true);
                    }
                )*

                Ok(false)
            }
        }

        impl zinq_parse::Parse for OpenDelim {
            #[inline]
            fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut zinq_parse::ZinqParser) -> zinq_error::Result<Self> {
                $(
                    if let Ok(ok) = parser.peek::<$name>(cursor) && ok {
                        return Ok(parser.parse::<$name>(cursor)?.into());
                    }
                )*

                Err(cursor.error(zinq_error::NOT_FOUND, &format!("unexpected token '{}'", *cursor.peek()? as char)))
            }
        }

        impl zinq_parse::Spanned for OpenDelim {
            fn span(&self) -> zinq_parse::Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }
        }

        impl From<OpenDelim> for $crate::Delim {
            #[inline]
            fn from(value: OpenDelim) -> Self {
                Self::Open(value.into())
            }
        }

        impl From<OpenDelim> for $crate::Token {
            #[inline]
            fn from(value: OpenDelim) -> Self {
                Self::Delim(value.into())
            }
        }

        impl std::fmt::Display for OpenDelim {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                }
            }
        }

        impl $crate::ToTokens for OpenDelim {
            fn to_tokens(&self) -> zinq_error::Result<$crate::TokenStream> {
                Ok($crate::Token::Delim($crate::Delim::Open(self.clone())).into())
            }
        }

        $(
            #[doc = "`"]
            #[doc = $token]
            #[doc = "`"]
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct $name {
                span: zinq_parse::Span,
            }

            impl $name {
                pub fn name(&self) -> &'static str {
                    stringify!($name)
                }
            }

            impl zinq_parse::Peek for $name {
                #[inline]
                fn peek(cursor: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
                    Ok(cursor.peek_n($token.len())? == $token.as_bytes())
                }
            }

            impl zinq_parse::Parse for $name {
                #[inline]
                fn parse(cursor: &mut zinq_parse::Cursor, _: &mut zinq_parse::ZinqParser) -> zinq_error::Result<Self> {
                    if !(cursor.next_n($token.len())?.span() == &$token.as_bytes()) {
                        return Err(cursor.error(zinq_error::NOT_FOUND, &format!("expected '{}'", $token)));
                    }

                    Ok(Self {
                        span: cursor.span().clone(),
                    })
                }
            }

            impl From<$name> for OpenDelim {
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
                    Self::Open(OpenDelim::$name(value))
                }
            }

            impl From<$name> for $crate::Token {
                #[inline]
                fn from(value: $name) -> Self {
                    Self::Delim($crate::Delim::Open(OpenDelim::$name(value)))
                }
            }

            impl zinq_parse::Spanned for $name {
                fn span(&self) -> zinq_parse::Span {
                    self.span.clone()
                }
            }

            impl $crate::Delim {
                pub fn $is_method(&self) -> bool {
                    match self {
                        Self::Open(delim) => delim.$is_method(),
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

            impl $crate::ToTokens for $name {
                fn to_tokens(&self) -> zinq_error::Result<$crate::TokenStream> {
                    Ok($crate::Token::Delim($crate::Delim::Open(OpenDelim::$name(self.clone()))).into())
                }
            }
        )*

        #[cfg(test)]
        mod tests {
            use zinq_error::Result;
            use zinq_parse::Span;

            $(
                #[test]
                fn $is_method() -> Result<()> {
                    let mut cursor = Span::from_str($token).cursor();
                    let mut parser = zinq_parse::ZinqParser;
                    let token = parser.parse::<$crate::Token>(&mut cursor)?;

                    debug_assert!(token.is_delim());
                    debug_assert!(token.$is_method());
                    debug_assert_eq!(token.to_string(), $token);

                    Ok(())
                }
            )*
        }
    };
}

define_open_delimiters! {
    "{",    pub struct LBrace,      is_left_brace,
    "[",    pub struct LBracket,    is_left_bracket,
    "(",    pub struct LParen,      is_left_paren
}
