use crate::Span;

///
/// ## Cursor
/// a mutable iterator/parser
///
#[derive(Debug, Default, Clone)]
pub struct Cursor {
    span: Span,
}

impl Cursor {
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

    #[inline]
    pub fn next(self) -> Self {
        let mut next = self.clone();
        next.span.end.index += 1;
        next.span.end.column += 1;

        if next.span.last() == b'\n' {
            next.span.end.line += 1;
            next.span.end.column = 0;
        }

        next
    }
}

impl From<Span> for Cursor {
    #[inline]
    fn from(span: Span) -> Self {
        Self { span }
    }
}

impl std::fmt::Display for Cursor {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl std::ops::Deref for Cursor {
    type Target = Span;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.span
    }
}
