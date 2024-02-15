use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Continue, Semicolon};

#[derive(Debug)]
pub struct ContinueStmt {
    span: Span,
}

span!(ContinueStmt);

impl Parse for ContinueStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#continue: Continue = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(r#continue.span(), semicolon.span()),
        })
    }
}

impl DisplayTree for ContinueStmt {
    fn display(&self, layer: usize) {
        branch("ContinueStmt", layer);
    }
}
