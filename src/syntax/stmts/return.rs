use crate::syntax::display_tree::{branch, DisplayTree};
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::symbols::r#return::Return;
use crate::syntax::symbols::semicolon::Semicolon;

pub struct StmtReturn {
    _return: Return,
    expr: Expr,
    _semicolon: Semicolon,
}

impl Parse for StmtReturn {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtReturn {
            _return: input.parse()?,
            expr: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtReturn {
    fn display(&self, layer: usize) {
        branch("ReturnStmt", layer);
        self.expr.display(layer + 1);
    }
}
