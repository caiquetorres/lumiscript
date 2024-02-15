use crate::parser::ParseError;
use crate::parser::ParseStream;

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError>;
}
