use crate::Result;

pub trait Token: Sized {
    fn parse() -> Result<Self>;
}
