use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::symbols::Ident;

pub struct IdentExpr {
    ident: Ident,
}

impl IdentExpr {
    pub(crate) fn new(ident: Ident) -> Self {
        Self { ident }
    }

    pub fn span(&self) -> &Span {
        self.ident.span()
    }
}

impl DisplayTree for IdentExpr {
    fn display(&self, layer: usize) {
        branch(&format!("IdentExpr: {}", self.ident.source_text()), layer);
    }
}
