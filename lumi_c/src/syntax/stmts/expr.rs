use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::semicolon::Semicolon;

pub struct StmtExpr {
    expr: Expr,
    _semicolon: Semicolon,
}

impl StmtExpr {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for StmtExpr {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Self {
            expr: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtExpr {
    fn display(&self, layer: usize) {
        self.expr.display(layer);
    }
}
