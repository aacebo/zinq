use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
    span: Option<proc_macro2::Span>,
    inner: Option<Arc<dyn std::error::Error>>,
}

impl Error {
    pub fn new(message: &str) -> Self {
        return Self {
            message: message.to_string(),
            span: None,
            inner: None,
        };
    }

    pub fn message(&self) -> &str {
        return &self.message;
    }

    pub fn span(&self) -> Option<proc_macro2::Span> {
        return self.span;
    }

    pub fn with_span(&self, value: proc_macro2::Span) -> Self {
        let mut next = self.clone();
        next.span = Some(value);
        return next;
    }

    pub fn with_inner_error<Err: std::error::Error + 'static>(&self, value: Err) -> Self {
        let mut next = self.clone();
        next.inner = Some(Arc::new(value));
        return next;
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)?;

        if let Some(inner) = &self.inner {
            write!(f, "{}", inner)?;
        }

        Ok(())
    }
}
