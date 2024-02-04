use crate::scanner::token::TokenKind;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::span::Span;
use crate::token;

pub struct Class {
    _span: Span,
}

impl Parse for Class {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Class {
            _span: Span::from_token(input.expect(token!(class))?),
        })
    }
}
