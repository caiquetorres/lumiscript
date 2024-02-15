use crate::syntax::display_tree::DisplayTree;
use crate::syntax::symbols::LeftParen;
use crate::syntax::symbols::RightParen;

use super::expr::Expr;

pub struct ExprParen {
    pub left_paren: LeftParen,
    pub expr: Box<Expr>,
    pub right_paren: RightParen,
}

impl DisplayTree for ExprParen {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}
