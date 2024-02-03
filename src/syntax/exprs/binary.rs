use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

use super::expr::Expr;

pub struct BinOp {
    span: Span,
}

impl BinOp {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for BinOp {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(BinOp {
            span: Span::from_token(input.next()),
        })
    }
}

impl DisplayTree for BinOp {
    fn display(&self, layer: usize) {
        branch(&format!("UnaryOp: {}", self.name()), layer)
    }
}

pub struct ExprBinary {
    pub left: Box<Expr>,
    pub operator: BinOp,
    pub right: Box<Expr>,
}

impl Parse for ExprBinary {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ExprBinary {
            left: input.parse()?,
            operator: input.parse()?,
            right: input.parse()?,
        })
    }
}

impl DisplayTree for ExprBinary {
    fn display(&self, layer: usize) {
        branch("BinaryExpr", layer);
        self.left.display(layer + 1);
        self.operator.display(layer + 1);
        self.right.display(layer + 1);
    }
}
