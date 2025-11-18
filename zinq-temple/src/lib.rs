mod arguments;
mod error;
mod parameter;

pub use arguments::*;
pub use error::*;
pub use parameter::*;

pub trait Render {
    fn render(&self, args: Arguments) -> Result<String, RenderError>;
}

pub trait Parse {
    fn parse(&self, input: &str) -> Result<Box<dyn Render>, ParseError>;
}
