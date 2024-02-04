use crate::scanner::token::TokenKind;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::span::Span;
use crate::token;

pub struct For {
    _span: Span,
}

impl Parse for For {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(For {
            _span: Span::from_token(input.expect(token!(for))?),
        })
    }
}
