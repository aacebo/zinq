use crate::parse::token::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arg {
    pub name: Token,
    pub colon: Token,
    pub value: Token,
}

impl Arg {
    pub fn parse(scan: &mut crate::parse::bytes::Scanner<'_>) -> Result<Self, crate::ParseError> {
        let name = Text::parse(scan)?;

        if !name.is_ident() {
            return Err(crate::ParseError::new("expected argument name").position(scan.position()));
        }

        let colon = Token::parse(scan)?;

        if !name.is_colon() {
            return Err(crate::ParseError::new("expected ':'").position(scan.position()));
        }

        return Ok(Self {
            name,
            colon,
            value: Token::parse(scan)?,
        });
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}{} {}", self.name, self.colon, self.value);
    }
}

impl PartialEq<super::Expression> for Arg {
    fn eq(&self, other: &super::Expression) -> bool {
        return match other {
            super::Expression::Arg(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Expression> for Arg {
    fn into(self) -> super::Expression {
        return super::Expression::Arg(self.clone());
    }
}
