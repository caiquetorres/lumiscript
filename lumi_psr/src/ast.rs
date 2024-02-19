use lumi_lxr::token::TokenKind;

use crate::display_tree::DisplayTree;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::stmts::Stmt;

#[derive(Debug)]
pub struct Ast {
    stmts: Vec<Stmt>,
}

impl Ast {
    pub fn stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }
}

impl Parse for Ast {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::Eof {
            Ok(Self { stmts: vec![] })
        } else {
            let mut stmts = vec![];
            while input.peek().kind() != TokenKind::Eof {
                stmts.push(input.parse()?);
            }
            Ok(Self { stmts })
        }
    }
}

impl DisplayTree for Ast {
    fn display(&self, layer: usize) {
        for stmt in &self.stmts {
            stmt.display(layer)
        }
    }
}
