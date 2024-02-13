use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::span::Span;
use crate::token;

pub struct Break {
    _span: Span,
}

impl Parse for Break {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Break {
            _span: Span::from_token(input.expect(token!(break))?),
        })
    }
}
