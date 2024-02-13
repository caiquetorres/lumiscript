use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Extern {
    span: Span,
}

impl Extern {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Extern {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Extern {
            span: Span::from_token(input.expect(token!(extern))?),
        })
    }
}

impl Parse for Option<Extern> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == token!(extern) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}
