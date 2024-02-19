use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::symbols::{Dot, Ident};

use super::Expr;

#[derive(Debug)]
pub struct GetExpr {
    span: Span,
    expr: Box<Expr>,
    ident: Ident,
}

impl GetExpr {
    pub(crate) fn new(expr: Expr, _dot: Dot, ident: Ident) -> Self {
        Self {
            span: Span::range(expr.span(), ident.span()),
            expr: Box::new(expr),
            ident,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn expr(&self) -> &Expr {
        self.expr.as_ref()
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl DisplayTree for GetExpr {
    fn display(&self, layer: usize) {
        branch("GetExpr", layer);
        self.ident.display(layer + 1);
        self.expr.display(layer + 1);
    }
}
