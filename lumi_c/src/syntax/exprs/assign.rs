use lumi_lxr::span::Span;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;

use super::expr::Expr;

pub struct AssignOp {
    span: Span,
}

impl AssignOp {
    pub fn source_text(&self) -> String {
        self.span.source_text()
    }
}

impl Parse for AssignOp {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(AssignOp {
            span: Span::from(input.next().span()),
        })
    }
}

impl DisplayTree for AssignOp {
    fn display(&self, layer: usize) {
        branch(&format!("UnaryOp: {}", self.source_text()), layer)
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
