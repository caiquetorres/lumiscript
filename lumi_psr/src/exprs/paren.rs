use lumi_lxr::span::Span;

use crate::{
    display_tree::DisplayTree,
    symbols::{LeftParen, RightParen},
};

use super::Expr;

pub struct ParenExpr {
    span: Span,
    expr: Box<Expr>,
}

impl ParenExpr {
    pub(crate) fn new(left_paren: LeftParen, expr: Expr, right_paren: RightParen) -> Self {
        Self {
            span: Span::range(left_paren.span(), right_paren.span()),
            expr: Box::new(expr),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl DisplayTree for ParenExpr {
    fn display(&self, layer: usize) {
        self.expr.display(layer)
    }
}
