use crate::parse::token::{LeftBrace, RightBrace, Token};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object {
    pub open: Token,
    pub fields: Vec<super::Field>,
    pub close: Token,
}

impl Object {
    pub fn parse(scan: &mut crate::parse::bytes::Scanner<'_>) -> Result<Self, crate::ParseError> {
        let open = LeftBrace::parse(scan)?;
        let mut fields = vec![];

        while !scan.is_eof() && !scan.next_if(b'}') {
            fields.push(super::Field::parse(scan)?);
        }

        let close = RightBrace::parse(scan)?;

        return Ok(Self {
            open,
            fields,
            close,
        });
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.open)?;

        for (i, field) in self.fields.iter().enumerate() {
            write!(f, "{}", field)?;

            if i < self.fields.len() - 1 {
                write!(f, ", ")?;
            }
        }

        return write!(f, "{}", self.close);
    }
}

impl Into<super::Expression> for Object {
    fn into(self) -> super::Expression {
        return super::Expression::Object(self.clone());
    }
}

impl PartialEq<super::Expression> for Object {
    fn eq(&self, other: &super::Expression) -> bool {
        return match other {
            super::Expression::Object(v) => v.eq(self),
            _ => false,
        };
    }
}
