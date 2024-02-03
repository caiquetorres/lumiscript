use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::equal::Equal;
use crate::syntax::symbols::ident::Ident;
use crate::syntax::symbols::r#let::Let;
use crate::syntax::symbols::semicolon::Semicolon;

pub struct StmtLet {
    _let: Let,
    ident: Ident,
    ty: Option<Type>,
    _equal: Equal,
    expr: Expr,
    _semicolon: Semicolon,
}

impl Parse for StmtLet {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtLet {
            _let: input.parse()?,
            ident: input.parse()?,
            ty: input.parse()?,
            _equal: input.parse()?,
            expr: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtLet {
    fn display(&self, layer: usize) {
        branch("LetStmt", layer);
        self.ident.display(layer + 1);

        if let Some(ty) = &self.ty {
            ty.display(layer + 1);
        }

        self.expr.display(layer + 1);
    }
}
