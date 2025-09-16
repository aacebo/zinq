use crate::parse::token::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    pub name: Token,
    pub args: Option<Vec<super::Arg>>,
    pub object: Option<super::Object>,
}

impl Field {
    pub fn parse(scan: &mut crate::parse::bytes::Scanner<'_>) -> Result<Self, crate::ParseError> {
        let name = Text::parse(scan)?;

        if !name.is_ident() {
            return Err(crate::ParseError::new("expected field name").position(scan.position()));
        }

        let mut field = Self {
            name,
            args: None,
            object: None,
        };

        if scan.next_if(b'(') {
            let mut args = Vec::<super::Arg>::new();

            while !scan.is_eof() && !scan.next_if(b')') {
                args.push(super::Arg::parse(scan)?.into());
            }

            field.args = Some(args);
        }

        if scan.curr() == b'{' {
            field.object = Some(super::Object::parse(scan)?);
        }

        return Ok(field.into());
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(v) = &self.args {
            write!(f, "(")?;

            for (i, arg) in v.iter().enumerate() {
                write!(f, "{}", arg)?;

                if i < v.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            write!(f, ")")?;
        }

        if let Some(v) = &self.object {
            write!(f, " {}", v)?;
        }

        return Ok(());
    }
}

impl PartialEq<super::Expression> for Field {
    fn eq(&self, other: &super::Expression) -> bool {
        return match other {
            super::Expression::Field(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Expression> for Field {
    fn into(self) -> super::Expression {
        return super::Expression::Field(self.clone());
    }
}
