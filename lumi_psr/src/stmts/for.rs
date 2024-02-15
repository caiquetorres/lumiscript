use lumi_lxr::span;
use lumi_lxr::span::Span;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::ident;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{For, Ident, In};

use super::block::BlockStmt;

#[derive(Debug)]
pub struct ForStmt {
    span: Span,
    ident: Ident,
    iter: Expr,
    block: BlockStmt,
}

span!(ForStmt);
ident!(ForStmt);

impl ForStmt {
    pub fn iter(&self) -> &Expr {
        &self.iter
    }

    pub fn block(&self) -> &BlockStmt {
        &self.block
    }
}

impl Parse for ForStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#for: For = input.parse()?;
        let ident: Ident = input.parse()?;
        let _in: In = input.parse()?;
        let iter: Expr = Expr::parse_without_eager_brace(input)?;
        let block: BlockStmt = input.parse()?;
        Ok(Self {
            span: Span::range(r#for.span(), block.span()),
            ident,
            iter,
            block,
        })
    }
}

impl DisplayTree for ForStmt {
    fn display(&self, layer: usize) {
        branch("ForStmt", layer);
        self.ident.display(layer + 1);
        branch("Iter", layer + 1);
        self.iter.display(layer + 2);
        self.block.display(layer + 1);
    }
}
