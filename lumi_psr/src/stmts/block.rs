use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{LeftBrace, RightBrace};

use super::Stmt;

#[derive(Debug)]
pub struct BlockStmt {
    span: Span,
    stmts: Vec<Stmt>,
}

span!(BlockStmt);

impl BlockStmt {
    pub fn stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }
}

impl Parse for BlockStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let mut stmts = vec![];
        let left_brace: LeftBrace = input.parse()?;
        while input.peek().kind() != TokenKind::RightBrace && input.peek().kind() != TokenKind::Eof
        {
            stmts.push(input.parse()?);
        }
        let right_brace: RightBrace = input.parse()?;
        Ok(Self {
            span: Span::range(left_brace.span(), right_brace.span()),
            stmts,
        })
    }
}

impl DisplayTree for BlockStmt {
    fn display(&self, layer: usize) {
        branch("BlockStmt", layer);
        self.stmts.display(layer + 1);
    }
}
