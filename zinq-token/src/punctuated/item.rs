use zinq_parse::{Parse, Peek, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    Leading(Span, T, P),
    Final(Span, T, Option<P>),
}

impl<T, P> Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    pub fn is_final(&self) -> bool {
        match self {
            Self::Final(_, _, _) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> &T {
        match self {
            Self::Leading(_, v, _) => v,
            Self::Final(_, v, _) => v,
        }
    }
}

impl<T, P> From<(T, P)> for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn from(value: (T, P)) -> Self {
        Self::Leading(
            Span::from_bounds(value.0.span(), value.1.span()),
            value.0,
            value.1,
        )
    }
}

impl<T, P> From<T> for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn from(value: T) -> Self {
        Self::Final(value.span().clone(), value, None)
    }
}

impl<T, P> std::fmt::Display for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leading(span, _, _) => write!(f, "{}", span),
            Self::Final(span, _, _) => write!(f, "{}", span),
        }
    }
}

impl<T, P> Peek for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<T>(cursor).unwrap_or(false))
    }
}

impl<T, P> Parse for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let value = parser.parse::<T>(cursor)?;

        if parser.peek::<P>(cursor).unwrap_or(false) {
            let punct = parser.parse::<P>(cursor)?;
            let span = Span::from_bounds(value.span(), punct.span());

            if parser.peek::<T>(cursor).unwrap_or(false) {
                return Ok(Self::Leading(span, value, punct));
            }

            return Ok(Self::Final(span, value, Some(punct)));
        }

        Ok(Self::Final(value.span().clone(), value, None))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Leading(span, _, _) => span,
            Self::Final(span, _, _) => span,
        }
    }
}
