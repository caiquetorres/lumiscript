use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::dot::Dot;
use crate::syntax::symbols::ident::Ident;

use super::expr::Expr;

pub struct ExprGet {
    pub expr: Box<Expr>,
    pub dot: Dot,
    pub ident: Ident,
}

impl Parse for ExprGet {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Self {
            expr: input.parse()?,
            dot: input.parse()?,
            ident: input.parse()?,
        })
    }
}

impl DisplayTree for ExprGet {
    fn display(&self, layer: usize) {
        branch("GetExpr", layer);
        self.ident.display(layer + 1);
        self.expr.display(layer + 1);
    }
}
