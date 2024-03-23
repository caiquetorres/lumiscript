use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Return, Semicolon};

#[derive(Debug)]
pub struct ReturnStmt {
    pub(crate) span: Span,
    pub(crate) expr: Option<Expr>,
}

span!(ReturnStmt);

impl ReturnStmt {
    pub fn expr(&self) -> Option<&Expr> {
        self.expr.as_ref()
    }
}

impl Parse for ReturnStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#return: Return = input.parse()?;
        let expr = if input.peek().kind() != TokenKind::Semicolon {
            Some(input.parse::<Expr>()?)
        } else {
            None
        };
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
        if let Some(expr) = &self.expr {
            expr.display(layer + 1);
        }
    }
}
