use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Const {
    span: Span,
}

impl Const {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Const {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Const {
            span: Span::from_token(input.expect(token!(const))?),
        })
    }
}
