use crate::compile_error::CompileError;
use crate::ident;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

pub struct Ident {
    pub span: Span,
}

impl Ident {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for Ident {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Ident {
            span: Span::from_token(input.expect(ident!())?),
        })
    }
}

impl DisplayTree for Ident {
    fn display(&self, layer: usize) {
        branch(&format!("Ident: {}", self.name()), layer)
    }
}
