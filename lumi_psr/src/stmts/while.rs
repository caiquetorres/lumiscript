use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::While;

use super::block::BlockStmt;
use super::Stmt;

#[derive(Debug)]
pub struct WhileStmt {
    span: Span,
    cond: Expr,
    block: BlockStmt,
}

span!(WhileStmt);

impl WhileStmt {
    pub fn cond(&self) -> &Expr {
        &self.cond
    }

    pub fn stmts(&self) -> &Vec<Stmt> {
        self.block.stmts()
    }
}

impl Parse for WhileStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#while: While = input.parse()?;
        let cond: Expr = Expr::parse_without_eager_brace(input)?;
        let block: BlockStmt = input.parse()?;
        Ok(Self {
            span: Span::range(r#while.span(), block.span()),
            cond,
            block,
        })
    }
}

impl DisplayTree for WhileStmt {
    fn display(&self, layer: usize) {
        branch("WhileStmt", layer);
        branch("Condition", layer + 1);
        self.cond.display(layer + 2);
        self.block.display(layer + 1);
    }
}
