use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::symbols::Print;
use crate::syntax::symbols::Semicolon;

pub struct StmtPrint {
    _println: Print,
    expr: Expr,
    _semicolon: Semicolon,
}

impl StmtPrint {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for StmtPrint {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtPrint {
            _println: input.parse()?,
            expr: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtPrint {
    fn display(&self, layer: usize) {
        branch("PrintlnStmt", layer);
        self.expr.display(layer + 1);
    }
}
