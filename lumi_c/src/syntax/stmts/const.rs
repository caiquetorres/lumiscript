use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::Colon;
use crate::syntax::symbols::Const;
use crate::syntax::symbols::Equal;
use crate::syntax::symbols::Ident;
use crate::syntax::symbols::Semicolon;

pub struct ConstType {
    _colon: Colon,
    ty: Type,
}

impl Parse for ConstType {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(ConstType {
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Option<ConstType> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == TokenKind::Colon {
            Ok(Some(ConstType {
                _colon: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct StmtConst {
    _const: Const,
    ident: Ident,
    ty: Option<ConstType>,
    _equal: Equal,
    expr: Expr,
    _semicolon: Semicolon,
}

impl StmtConst {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for StmtConst {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtConst {
            _const: input.parse()?,
            ident: input.parse()?,
            ty: input.parse()?,
            _equal: input.parse()?,
            expr: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl DisplayTree for StmtConst {
    fn display(&self, layer: usize) {
        branch("ConstStmt", layer);
        self.ident.display(layer + 1);

        if let Some(ty) = &self.ty {
            ty.ty.display(layer + 1);
        }

        self.expr.display(layer + 1);
    }
}
