use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct LeftParen {
    _span: Span,
}

impl Parse for LeftParen {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(LeftParen {
            _span: Span::from_token(input.expect(token!('('))?),
        })
    }
}

pub struct RightParen {
    _span: Span,
}

impl Parse for RightParen {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(RightParen {
            _span: Span::from_token(input.expect(token!(')'))?),
        })
    }
}
