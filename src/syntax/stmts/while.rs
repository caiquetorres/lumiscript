use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::r#while::While;

use super::stmt::Stmt;

pub struct StmtWhile {
    _while: While,
    cond: Expr,
    stmt: Box<Stmt>,
}

impl Parse for StmtWhile {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtWhile {
            _while: input.parse()?,
            cond: input.parse()?,
            stmt: input.parse()?,
        })
    }
}

impl DisplayTree for StmtWhile {
    fn display(&self, layer: usize) {
        branch("WhileStmt", layer);
        branch("Condition", layer + 1);
        self.cond.display(layer + 2);
        self.stmt.display(layer + 1);
    }
}
