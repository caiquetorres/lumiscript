use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Return, Semicolon};

pub struct ReturnStmt {
    pub(crate) span: Span,
    pub(crate) expr: Expr,
}

span!(ReturnStmt);

impl ReturnStmt {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for ReturnStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#return: Return = input.parse()?;
        let expr: Expr = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(r#return.span(), semicolon.span()),
            expr,
        })
    }
}

impl DisplayTree for ReturnStmt {
    fn display(&self, layer: usize) {
        branch("ReturnStmt", layer);
        self.expr.display(layer + 1);
    }
}
