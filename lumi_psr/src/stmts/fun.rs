use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::param::Param;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Arrow, Extern, Fun, Ident, LeftParen, RightParen, Semicolon};
use crate::ty::Type;

use super::block::BlockStmt;

pub enum FunStmt {
    Default {
        span: Span,
        ident: Ident,
        params: Vec<Param>,
        return_ty: Option<Type>,
        block: BlockStmt,
    },
    Extern {
        span: Span,
        ident: Ident,
        params: Vec<Param>,
        return_ty: Option<Type>,
    },
}

impl FunStmt {
    pub fn span(&self) -> &Span {
        match self {
            Self::Extern { span, .. } => span,
            Self::Default { span, .. } => span,
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            Self::Extern { ident, .. } => ident,
            Self::Default { ident, .. } => ident,
        }
    }

    pub fn params(&self) -> &Vec<Param> {
        match self {
            Self::Extern { params, .. } => params,
            Self::Default { params, .. } => params,
        }
    }

    pub fn return_ty(&self) -> Option<&Type> {
        match self {
            Self::Extern { return_ty, .. } => return_ty.as_ref(),
            Self::Default { return_ty, .. } => return_ty.as_ref(),
        }
    }

    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default { .. })
    }

    pub fn is_extern(&self) -> bool {
        matches!(self, Self::Extern { .. })
    }
}

impl Parse for FunStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::Extern {
            let r#extern: Extern = input.parse()?;
            let _fun: Fun = input.parse()?;
            let ident: Ident = input.parse()?;
            let _left_paren: LeftParen = input.parse()?;
            let params: Vec<Param> = input.parse()?;
            let _right_paren: RightParen = input.parse()?;
            let return_ty: Option<Type> = if input.peek().kind() == TokenKind::MinusGreater {
                let _arrow: Arrow = input.parse()?;
                Some(input.parse()?)
            } else {
                None
            };
            let semicolon: Semicolon = input.parse()?;
            Ok(FunStmt::Extern {
                span: Span::range(r#extern.span(), semicolon.span()),
                ident,
                params,
                return_ty,
            })
        } else {
            let r#fun: Fun = input.parse()?;
            let ident: Ident = input.parse()?;
            let _left_paren: LeftParen = input.parse()?;
            let params: Vec<Param> = input.parse()?;
            let _right_paren: RightParen = input.parse()?;
            let return_ty: Option<Type> = if input.peek().kind() == TokenKind::MinusGreater {
                let _arrow: Arrow = input.parse()?;
                Some(input.parse()?)
            } else {
                None
            };
            let block: BlockStmt = input.parse()?;
            Ok(FunStmt::Default {
                span: Span::range(r#fun.span(), block.span()),
                ident,
                params,
                return_ty,
                block,
            })
        }
    }
}

impl DisplayTree for FunStmt {
    fn display(&self, layer: usize) {
        match self {
            Self::Default {
                ident,
                params,
                return_ty,
                block,
                ..
            } => {
                branch("FunStmt", layer);
                ident.display(layer + 1);
                params.display(layer + 1);
                if let Some(return_ty) = &return_ty {
                    return_ty.display(layer + 1);
                }
                block.display(layer + 1);
            }
            Self::Extern {
                ident,
                params,
                return_ty,
                ..
            } => {
                branch("ExternFunStmt", layer);
                ident.display(layer + 1);
                params.display(layer + 1);
                if let Some(return_ty) = &return_ty {
                    return_ty.display(layer + 1);
                }
            }
        }
    }
}
