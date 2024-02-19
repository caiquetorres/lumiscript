use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};

use super::Expr;

#[derive(Debug)]
pub struct BinaryOp {
    span: Span,
}

impl BinaryOp {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn source_text(&self) -> String {
        self.span.source_text()
    }
}

impl Parse for BinaryOp {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            span: Span::from(input.next().span()),
        })
    }
}

impl DisplayTree for BinaryOp {
    fn display(&self, layer: usize) {
        branch(&format!("BinaryOp: {}", self.source_text()), layer)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    span: Span,
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Self {
            span: Span::range(left.span(), right.span()),
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn left(&self) -> &Expr {
        self.left.as_ref()
    }

    pub fn op(&self) -> &BinaryOp {
        &self.op
    }

    pub fn right(&self) -> &Expr {
        self.right.as_ref()
    }
}

impl DisplayTree for BinaryExpr {
    fn display(&self, layer: usize) {
        branch("BinaryExpr", layer);
        self.left.display(layer + 1);
        self.op.display(layer + 1);
        self.right.display(layer + 1);
    }
}
