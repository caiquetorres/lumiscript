use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::colon::Colon;
use crate::syntax::symbols::equal::Equal;
use crate::syntax::symbols::ident::Ident;
use crate::syntax::symbols::r#let::Let;
use crate::syntax::symbols::semicolon::Semicolon;
use crate::token;

pub struct LetType {
    _colon: Colon,
    ty: Type,
}

impl Parse for LetType {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(LetType {
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Option<LetType> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(:) {
            Ok(Some(LetType {
                _colon: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct StmtLet {
    _let: Let,
    ident: Ident,
    ty: Option<LetType>,
    _equal: Equal,
    expr: Expr,
    _semicolon: Semicolon,
}

impl StmtLet {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
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
            ty.ty.display(layer + 1);
        }

        self.expr.display(layer + 1);
    }
}
