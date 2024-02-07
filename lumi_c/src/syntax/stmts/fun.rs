use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::arrow::Arrow;
use crate::syntax::symbols::colon::Colon;
use crate::syntax::symbols::fun::Fun;
use crate::syntax::symbols::ident::Ident;
use crate::syntax::symbols::paren::LeftParen;
use crate::syntax::symbols::paren::RightParen;
use crate::syntax::symbols::r#extern::Extern;
use crate::syntax::symbols::r#static::Static;
use crate::syntax::symbols::semicolon::Semicolon;
use crate::token;

use super::block::StmtBlock;

pub struct ParamType {
    _colon: Colon,
    ty: Type,
}

impl Parse for ParamType {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ParamType {
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Option<ParamType> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(:) {
            Ok(Some(ParamType {
                _colon: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct Param {
    ident: Ident,
    ty: ParamType,
}

impl Param {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &ParamType {
        &self.ty
    }
}

impl Parse for Param {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Param {
            ident: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Vec<Param> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut params = vec![];

        if input.peek() == token!(')') {
            Ok(params)
        } else {
            params.push(input.parse()?);

            while input.peek() == token!(,) {
                input.expect(token!(,))?;
                params.push(input.parse()?);
            }

            Ok(params)
        }
    }
}

impl DisplayTree for Param {
    fn display(&self, layer: usize) {
        branch("Param", layer);
        self.ident.display(layer + 1);
        self.ty.ty.display(layer + 1);
    }
}

impl DisplayTree for Vec<Param> {
    fn display(&self, layer: usize) {
        if !self.is_empty() {
            branch("Params", layer);
            for param in self {
                param.display(layer + 1);
            }
        }
    }
}

pub struct ReturnType {
    _arrow: Arrow,
    ty: Type,
}

impl Parse for ReturnType {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ReturnType {
            _arrow: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Option<ReturnType> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(->) {
            Ok(Some(ReturnType {
                _arrow: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub enum StmtFun {
    Default {
        r#static: Option<Static>,
        _fun: Fun,
        ident: Ident,
        _left_paren: LeftParen,
        params: Vec<Param>,
        _right_paren: RightParen,
        return_ty: Option<ReturnType>,
        block: StmtBlock,
    },
    Proto {
        _extern: Option<Extern>,
        r#static: Option<Static>,
        _fun: Fun,
        ident: Ident,
        _left_paren: LeftParen,
        params: Vec<Param>,
        _right_paren: RightParen,
        return_ty: Option<ReturnType>,
        _semicolon: Semicolon,
    },
}

impl Parse for StmtFun {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let _extern: Option<Extern> = input.parse()?;
        let r#static = input.parse()?;
        let _fun = input.parse()?;
        let ident = input.parse()?;
        let _left_paren = input.parse()?;
        let params = input.parse()?;
        let _right_paren = input.parse()?;
        let return_ty = input.parse()?;

        if _extern.is_some() {
            Ok(StmtFun::Proto {
                _extern,
                r#static,
                _fun,
                ident,
                _left_paren,
                params,
                _right_paren,
                return_ty,
                _semicolon: input.parse()?,
            })
        } else {
            if input.peek() == token!(;) {
                Ok(StmtFun::Proto {
                    _extern,
                    r#static,
                    _fun,
                    ident,
                    _left_paren,
                    params,
                    _right_paren,
                    return_ty,
                    _semicolon: input.parse()?,
                })
            } else {
                Ok(StmtFun::Default {
                    r#static,
                    _fun,
                    ident,
                    _left_paren,
                    params,
                    _right_paren,
                    return_ty,
                    block: input.parse()?,
                })
            }
        }
    }
}

impl Parse for Vec<StmtFun> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut functions = vec![];

        while input.peek() != token!('}') {
            functions.push(input.parse()?)
        }

        Ok(functions)
    }
}

impl DisplayTree for StmtFun {
    fn display(&self, layer: usize) {
        match self {
            Self::Default { r#static, .. } => {
                if let Some(_) = r#static {
                    branch("StaticFunStmt", layer);
                } else {
                    branch("FunStmt", layer);
                }
            }
            Self::Proto {
                _extern, r#static, ..
            } => {
                if let Some(_) = r#static {
                    if let Some(_) = _extern {
                        branch("ExternStaticProtoFunStmt", layer);
                    } else {
                        branch("StaticProtoFunStmt", layer);
                    }
                } else {
                    if let Some(_) = _extern {
                        branch("ExternProtoFunStmt", layer);
                    } else {
                        branch("ProtoFunStmt", layer);
                    }
                }
            }
        }

        match self {
            Self::Default {
                ident,
                params,
                return_ty,
                block,
                ..
            } => {
                ident.display(layer + 1);
                params.display(layer + 1);

                if let Some(return_ty) = &return_ty {
                    return_ty.ty.display(layer + 1);
                }

                block.display(layer + 1);
            }
            Self::Proto {
                ident,
                params,
                return_ty,
                ..
            } => {
                ident.display(layer + 1);
                params.display(layer + 1);

                if let Some(return_ty) = &return_ty {
                    return_ty.ty.display(layer + 1);
                }
            }
        }
    }
}

impl DisplayTree for Vec<StmtFun> {
    fn display(&self, layer: usize) {
        for fun in self {
            fun.display(layer);
        }
    }
}
