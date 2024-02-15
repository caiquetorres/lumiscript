use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::Semicolon;

pub struct ExprStmt {
    span: Span,
    expr: Expr,
}

span!(ExprStmt);

impl ExprStmt {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for ExprStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let expr: Expr = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(expr.span(), semicolon.span()),
            expr,
        })
    }
}

impl DisplayTree for ExprStmt {
    fn display(&self, layer: usize) {
        branch("ExprStmt", layer);
        self.expr.display(layer + 1);
    }
}
