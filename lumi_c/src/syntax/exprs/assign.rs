use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

use super::expr::Expr;

pub struct AssignOp {
    span: Span,
}

impl AssignOp {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for AssignOp {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(AssignOp {
            span: Span::from_token(input.next()),
        })
    }
}

impl DisplayTree for AssignOp {
    fn display(&self, layer: usize) {
        branch(&format!("UnaryOp: {}", self.name()), layer)
    }
}

pub struct ExprAssign {
    pub left: Box<Expr>,
    pub operator: AssignOp,
    pub right: Box<Expr>,
}

impl DisplayTree for ExprAssign {
    fn display(&self, layer: usize) {
        branch("AssignExpr", layer);
        self.left.display(layer + 1);
        self.operator.display(layer + 1);
        self.right.display(layer + 1);
    }
}
