use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};

use super::Expr;

#[derive(Debug)]
pub struct UnaryOp {
    span: Span,
}

impl UnaryOp {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn source_text(&self) -> String {
        self.span.source_text()
    }
}

impl Parse for UnaryOp {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        Ok(Self {
            span: Span::from(input.next().span()),
        })
    }
}

impl DisplayTree for UnaryOp {
    fn display(&self, layer: usize) {
        branch(&format!("UnaryOp: {}", self.source_text()), layer)
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    span: Span,
    op: UnaryOp,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub(crate) fn new(operator: UnaryOp, expr: Expr) -> Self {
        Self {
            span: Span::range(operator.span(), expr.span()),
            op: operator,
            expr: Box::new(expr),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl DisplayTree for UnaryExpr {
    fn display(&self, layer: usize) {
        branch("UnaryExpr", layer);
        self.op.display(layer + 1);
        self.expr.display(layer + 1);
    }
}
