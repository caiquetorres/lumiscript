use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::paren::LeftParen;
use crate::syntax::symbols::paren::RightParen;

use super::expr::Expr;

pub struct ExprCall {
    pub callee: Box<Expr>,
    pub left_paren: LeftParen,
    pub args: Vec<Expr>,
    pub right_paren: RightParen,
}

impl Parse for ExprCall {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Self {
            callee: input.parse()?,
            left_paren: input.parse()?,
            args: input.parse()?,
            right_paren: input.parse()?,
        })
    }
}

impl DisplayTree for ExprCall {
    fn display(&self, layer: usize) {
        branch("CallExpr", layer);

        if !self.args.is_empty() {
            branch("Args", layer + 1);
            self.args.display(layer + 2);
        }

        self.callee.display(layer + 1);
    }
}
