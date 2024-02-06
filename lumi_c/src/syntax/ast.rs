use crate::scanner::token::TokenKind;
use crate::token;

use super::display_tree::DisplayTree;
use super::parse::Parse;
use super::parse::ParseStream;
use super::stmts::stmt::Stmt;

pub struct Ast {
    stmts: Vec<Stmt>,
}

impl Ast {
    pub fn stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }
}

impl Parse for Ast {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut stmts = vec![];

        while input.peek() != token!(eof) {
            if let Ok(stmt) = input.parse::<Stmt>() {
                stmts.push(stmt);
            } else {
                break;
            }
        }

        Ok(Self { stmts })
    }
}

impl DisplayTree for Ast {
    fn display(&self, layer: usize) {
        for stmt in &self.stmts {
            stmt.display(layer);
        }
    }
}
