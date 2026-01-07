use zinq_parse::{Parse, Peek, Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    Leading(T, P),
    Final(T, Option<P>),
}

impl<T, P> Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    pub fn is_final(&self) -> bool {
        match self {
            Self::Final(_, _) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> &T {
        match self {
            Self::Leading(v, _) => v,
            Self::Final(v, _) => v,
        }
    }
}

impl<T, P> From<(T, P)> for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn from(value: (T, P)) -> Self {
        Self::Leading(value.0, value.1)
    }
}

impl<T, P> From<T> for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn from(value: T) -> Self {
        Self::Final(value, None)
    }
}

impl<T, P> std::fmt::Display for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
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

            if parser.peek::<T>(cursor).unwrap_or(false) {
                return Ok(Self::Leading(value, punct));
            }

            return Ok(Self::Final(value, Some(punct)));
        }

        Ok(Self::Final(value, None))
    }
}

impl<T, P> Spanned for Item<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Leading(v, p) => Span::join(v.span(), p.span()),
            Self::Final(v, p) => match p {
                None => v.span(),
                Some(vp) => Span::join(v.span(), vp.span()),
            },
        }
    }
}
