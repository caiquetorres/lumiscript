use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::paren::LeftParen;
use crate::syntax::symbols::paren::RightParen;

use super::expr::Expr;

pub struct ExprParen {
    _left_paren: LeftParen,
    expr: Box<Expr>,
    _right_paren: RightParen,
}

impl Parse for ExprParen {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ExprParen {
            _left_paren: input.parse()?,
            expr: input.parse()?,
            _right_paren: input.parse()?,
        })
    }
}

impl DisplayTree for ExprParen {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}
