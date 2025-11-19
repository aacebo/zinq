use parse_format::{Argument, ParseMode, Parser, Piece};

#[derive(Debug, Clone)]
pub struct Template<'a> {
    pub parts: Vec<Piece<'a>>,
    pub arguments: Vec<Argument<'a>>,
}

impl<'a> Template<'a> {
    pub fn parse(fmt: &str) -> Result<Template<'_>, syn::Error> {
        let mut parser = Parser::new(fmt, None, None, false, ParseMode::Format);
        let mut template = Template {
            parts: vec![],
            arguments: vec![],
        };

        while let Some(piece) = parser.next() {
            template.parts.push(piece.clone());

            if let Piece::NextArgument(arg) = &piece {
                template.arguments.push(arg.clone());
            }
        }

        if let Some(err) = parser.errors.first() {
            let error = syn::Error::new(proc_macro2::Span::call_site(), err.to_string());
            return Err(error);
        }

        return Ok(template);
    }
}

impl<'a> std::fmt::Display for Template<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in self.parts.iter() {
            write!(f, "{}", part)?;
        }

        return Ok(());
    }
}
