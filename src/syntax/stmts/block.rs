use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::brace::LeftBrace;
use crate::syntax::symbols::brace::RightBrace;

use super::stmt::Stmt;

pub struct StmtBlock {
    _left_brace: LeftBrace,
    stmts: Vec<Box<Stmt>>,
    _right_brace: RightBrace,
}

impl Parse for StmtBlock {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtBlock {
            _left_brace: input.parse()?,
            stmts: input.parse()?,
            _right_brace: input.parse()?,
        })
    }
}

impl DisplayTree for StmtBlock {
    fn display(&self, layer: usize) {
        branch("BlockStmt", layer);
        self.stmts.display(layer + 1);
    }
}