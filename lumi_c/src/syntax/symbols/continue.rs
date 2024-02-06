use crate::scanner::token::TokenKind;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::span::Span;
use crate::token;

pub struct Continue {
    _span: Span,
}

impl Parse for Continue {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Continue {
            _span: Span::from_token(input.expect(token!(continue))?),
        })
    }
}
