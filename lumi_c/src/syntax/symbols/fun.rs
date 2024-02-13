use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Fun {
    span: Span,
}

impl Fun {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Fun {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Fun {
            span: Span::from_token(input.expect(token!(fun))?),
        })
    }
}
