use crate::compile_error::CompileError;
use crate::syntax::display_tree::{branch, DisplayTree};
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::{Parse, ParseStream};
use crate::syntax::symbols::For;
use crate::syntax::symbols::Ident;
use crate::syntax::symbols::In;

use super::stmt::Stmt;

pub struct StmtFor {
    _for: For,
    ident: Ident,
    _in: In,
    iter: Expr,
    stmt: Box<Stmt>,
}

impl Parse for StmtFor {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Self {
            _for: input.parse()?,
            ident: input.parse()?,
            _in: input.parse()?,
            iter: Expr::parse_without_eager_brace(input)?,
            stmt: input.parse()?,
        })
    }
}

impl DisplayTree for StmtFor {
    fn display(&self, layer: usize) {
        branch("ForStmt", layer);
        self.ident.display(layer + 1);
        branch("Iter", layer + 1);
        self.iter.display(layer + 2);
        self.stmt.display(layer + 1);
    }
}
