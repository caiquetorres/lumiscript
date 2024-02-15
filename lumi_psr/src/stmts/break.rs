use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Break, Semicolon};

#[derive(Debug)]
pub struct BreakStmt {
    span: Span,
}

span!(BreakStmt);

impl Parse for BreakStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#break: Break = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(r#break.span(), semicolon.span()),
        })
    }
}

impl DisplayTree for BreakStmt {
    fn display(&self, layer: usize) {
        branch("BreakStmt", layer);
    }
}
