use crate::Span;

///
/// ## ByteParser
/// the default implementation of a
/// `Parser` that can iterate/parse some
/// `Bytes`
///
#[derive(Debug, Default, Clone)]
pub struct ByteParser {
    span: Span,
}

impl ByteParser {
    #[inline]
    pub fn new() -> Self {
        Self {
            span: Span::default(),
        }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl From<Span> for ByteParser {
    #[inline]
    fn from(value: Span) -> Self {
        Self { span: value }
    }
}

impl std::ops::Deref for ByteParser {
    type Target = Span;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.span
    }
}
