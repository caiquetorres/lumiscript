use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Trait {
    span: Span,
}

impl Trait {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Trait {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Trait {
            span: Span::from_token(input.expect(token!(trait))?),
        })
    }
}
