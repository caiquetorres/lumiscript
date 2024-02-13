use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct LeftBrace {
    _span: Span,
}

impl Parse for LeftBrace {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(LeftBrace {
            _span: Span::from_token(input.expect(token!('{'))?),
        })
    }
}

pub struct RightBrace {
    _span: Span,
}

impl Parse for RightBrace {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(RightBrace {
            _span: Span::from_token(input.expect(token!('}'))?),
        })
    }
}
