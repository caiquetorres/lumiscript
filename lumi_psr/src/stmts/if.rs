use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::If;

use super::block::BlockStmt;
use super::Stmt;

pub struct IfStmt {
    span: Span,
    cond: Expr,
    block: BlockStmt,
}

span!(IfStmt);

impl IfStmt {
    pub fn cond(&self) -> &Expr {
        &self.cond
    }

    pub fn stmts(&self) -> &Vec<Stmt> {
        self.block.stmts()
    }
}

impl Parse for IfStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#if: If = input.parse()?;
        let cond: Expr = Expr::parse_without_eager_brace(input)?;
        let block: BlockStmt = input.parse()?;
        Ok(Self {
            span: Span::range(r#if.span(), block.span()),
            cond,
            block,
        })
    }
}

impl DisplayTree for IfStmt {
    fn display(&self, layer: usize) {
        branch("IfStmt", layer);
        branch("Condition", layer + 1);
        self.cond.display(layer + 2);
        self.block.display(layer + 1);
    }
}
