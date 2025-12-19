macro_rules! define_keywords {
    ($($token:literal pub struct $name:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl $crate::Token for Keyword {
            fn peek(cursor: $crate::Cursor) -> bool {
                $(
                    if $name::peek(cursor.clone()) {
                        return true;
                    }
                )*

                false
            }

            fn parse(cursor: $crate::Cursor) -> zinq_error::Result<Self> {
                $(
                    if $name::peek(cursor.clone()) {
                        return Ok($name::parse(cursor.clone().into())?.into());
                    }
                )*

                Err(cursor.error(&format!("invalid token")))
            }

            fn span(&self) -> &$crate::Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }
        }

        impl From<Keyword> for $crate::tokens::Any {
            fn from(value: Keyword) -> Self {
                Self::Keyword(value)
            }
        }

        $(
            #[doc = concat!('`', $token, '`')]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                span: $crate::Span,
            }

            impl $name {
                #[inline]
                pub fn span(&self) -> &$crate::Span {
                    &self.span
                }
            }

            impl $crate::Token for $name {
                fn peek(cursor: $crate::Cursor) -> bool {
                    cursor.matches($token.as_bytes())
                }

                fn parse(cursor: $crate::Cursor) -> zinq_error::Result<Self> {
                    if !cursor.matches($token.as_bytes()) {
                        return Err(cursor.error(&format!("expected '{}'", $token)));
                    }

                    Ok(Self {
                        span: cursor.span().clone(),
                    })
                }

                fn span(&self) -> &$crate::Span {
                    &self.span
                }
            }

            impl From<$name> for Keyword {
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
