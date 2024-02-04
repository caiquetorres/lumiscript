use crate::scanner::token::TokenKind;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;
use crate::token;

pub struct Equal {
    span: Span,
}

impl Equal {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Equal {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Equal {
            span: Span::from_token(input.expect(token!(=))?),
        })
    }
}
