use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::span::Span;

use super::expr::Expr;

pub struct UnaryOp {
    span: Span,
}

impl UnaryOp {
    pub fn name(&self) -> String {
        self.span.source_text.clone()
    }
}

impl Parse for UnaryOp {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(UnaryOp {
            span: Span::from_token(input.next()),
        })
    }
}

impl DisplayTree for UnaryOp {
    fn display(&self, layer: usize) {
        branch(&format!("UnaryOp: {}", self.name()), layer)
    }
}

pub struct ExprUnary {
    pub operator: UnaryOp,
    pub expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(ExprUnary {
            operator: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl DisplayTree for ExprUnary {
    fn display(&self, layer: usize) {
        branch("UnaryExpr", layer);
        self.operator.display(layer + 1);
        self.expr.display(layer + 1);
    }
}
