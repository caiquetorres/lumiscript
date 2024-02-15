use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::ident;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::Ident;

#[derive(Debug)]
pub struct Type {
    span: Span,
    ident: Ident,
}

span!(Type);
ident!(Type);

impl Parse for Type {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let ident: Ident = input.parse()?;
        Ok(Self {
            span: ident.span().clone(),
            ident,
        })
    }
}

impl DisplayTree for Type {
    fn display(&self, layer: usize) {
        branch(&format!("Type: {}", self.ident.source_text(),), layer);
    }
}
