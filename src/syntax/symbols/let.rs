use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Let {
    span: Span,
}

impl Let {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Let {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Let {
            span: Span::from_token(input.expect(token!(let))?),
        })
    }
}
