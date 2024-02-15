use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::exprs::Expr;
use crate::ident;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Colon, Const, Equal, Ident, Semicolon};
use crate::ty::Type;

pub struct ConstType {
    span: Span,
    ty: Type,
}

span!(ConstType);

impl Parse for ConstType {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let colon: Colon = input.parse()?;
        let ty: Type = input.parse()?;
        Ok(Self {
            span: Span::range(colon.span(), ty.span()),
            ty,
        })
    }
}

impl Parse for Option<ConstType> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::Colon {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

pub struct ConstStmt {
    span: Span,
    ident: Ident,
    ty: Option<ConstType>,
    expr: Expr,
}

span!(ConstStmt);
ident!(ConstStmt);

impl ConstStmt {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for ConstStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#let: Const = input.parse()?;
        let ident: Ident = input.parse()?;
        let ty: Option<ConstType> = input.parse()?;
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

impl DisplayTree for ConstStmt {
    fn display(&self, layer: usize) {
        branch("ConstStmt", layer);
        self.ident.display(layer + 1);
        if let Some(ty) = &self.ty {
            ty.ty.display(layer + 1);
        }
        self.expr.display(layer + 1);
    }
}
