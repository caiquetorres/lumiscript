use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::branch;
use crate::display_tree::DisplayTree;
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::ParseError;
use crate::parser::ParseStream;
use crate::symbols::Println;
use crate::symbols::Semicolon;

#[derive(Debug)]
pub struct PrintlnStmt {
    span: Span,
    expr: Expr,
}

span!(PrintlnStmt);

impl PrintlnStmt {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for PrintlnStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let println: Println = input.parse()?;
        let expr = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(println.span(), semicolon.span()),
            expr,
        })
    }
}

impl DisplayTree for PrintlnStmt {
    fn display(&self, layer: usize) {
        branch("PrintlnStmt", layer);
        self.expr.display(layer + 1);
    }
}
