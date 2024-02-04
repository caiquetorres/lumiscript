use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::paren::LeftParen;
use crate::syntax::symbols::paren::RightParen;

use super::expr::Expr;

pub struct ExprParen {
    pub left_paren: LeftParen,
    pub expr: Box<Expr>,
    pub right_paren: RightParen,
}

impl Parse for ExprParen {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ExprParen {
            left_paren: input.parse()?,
            expr: input.parse()?,
            right_paren: input.parse()?,
        })
    }
}

impl DisplayTree for ExprParen {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}
