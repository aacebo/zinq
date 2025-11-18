mod parse_error;
mod render_error;

pub use parse_error::*;
pub use render_error::*;

#[derive(Clone, Debug)]
pub enum Error {
    Parse(ParseError),
    Render(RenderError),
}
