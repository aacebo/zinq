use parse_format::{ParseMode, Parser, Piece, Position};

#[derive(Debug)]
pub enum Part<'a> {
    Literal(&'a str),
    Arg {
        // one of `{}`, `{0}`, `{name}`
        position: Position<'a>,
        // you can dig into this for precision, width, etc.
        format: parse_format::FormatSpec<'a>,
    },
}

pub fn parse(fmt: &str) -> Result<Vec<Part<'_>>, syn::Error> {
    // style: None        -> no special style (see rustc source if you care)
    // snippet: None      -> we just pass the raw string
    // append_newline: false
    // mode: ParseMode::Format -> "normal" format string, like format_args!
    let mut parser = Parser::new(fmt, None, None, false, ParseMode::Format);
    let mut pieces = Vec::new();

    while let Some(piece) = parser.next() {
        match piece {
            Piece::String(s) => {
                pieces.push(Part::Literal(s));
            }
            Piece::NextArgument(arg) => {
                pieces.push(Part::Arg {
                    position: arg.position,
                    format: arg.format,
                });

                // match arg.position {
                //     Position::ArgumentNamed(name) => {

                //     },
                //     Position::ArgumentIs(index) => {

                //     },
                //     Position::ArgumentImplicitlyIs(index) => {

                //     },
                // };
            }
        }
    }

    if let Some(err) = parser.errors.first() {
        let error = syn::Error::new(proc_macro2::Span::call_site(), err.to_string());
        return Err(error);
    }

    return Ok(pieces);
}
