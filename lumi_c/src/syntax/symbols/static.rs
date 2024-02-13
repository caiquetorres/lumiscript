use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Static {
    span: Span,
}

impl Static {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Static {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Static {
            span: Span::from_token(input.expect(token!(static))?),
        })
    }
}

impl Parse for Option<Static> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == token!(static) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}
