use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::{branch, DisplayTree};
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::symbols::r#return::Return;
use crate::syntax::symbols::semicolon::Semicolon;
use crate::token;

pub struct StmtReturn {
    _return: Return,
    expr: Option<Expr>,
    _semicolon: Semicolon,
}

impl StmtReturn {
    pub fn expr(&self) -> Option<&Expr> {
        self.expr.as_ref()
    }
}

impl Parse for StmtReturn {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtReturn {
            _return: input.parse()?,
            expr: {
                if input.peek() != token!(;) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtReturn {
    fn display(&self, layer: usize) {
        branch("ReturnStmt", layer);

        if let Some(expr) = &self.expr {
            expr.display(layer + 1);
        }
    }
}
