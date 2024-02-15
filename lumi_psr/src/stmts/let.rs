use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::ident;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Equal, Ident, Let, Semicolon};
use crate::ty::Type;

#[derive(Debug)]
pub struct LetStmt {
    span: Span,
    ident: Ident,
    ty: Option<Type>,
    expr: Expr,
}

span!(LetStmt);
ident!(LetStmt);

impl LetStmt {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for LetStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#let: Let = input.parse()?;
        let ident: Ident = input.parse()?;

        let ty = if input.peek().kind() == TokenKind::Colon {
            Some(input.parse()?)
        } else {
            None
        };

        let _equal: Equal = input.parse()?;
        let expr: Expr = input.parse()?;
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(r#let.span(), semicolon.span()),
            ident,
            ty,
            expr,
        })
    }
}

impl DisplayTree for LetStmt {
    fn display(&self, layer: usize) {
        branch("LetStmt", layer);
        self.ident.display(layer + 1);
        if let Some(ty) = &self.ty {
            ty.display(layer + 1);
        }
        self.expr.display(layer + 1);
    }
}
