use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::span::Span;
use crate::token;

pub struct While {
    _span: Span,
}

impl Parse for While {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(While {
            _span: Span::from_token(input.expect(token!(while))?),
        })
    }
}
