use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::param::Param;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{
    Arrow, Extern, For, Fun, Ident, Impl, LeftBrace, LeftParen, RightBrace, RightParen, Semicolon,
};
use crate::ty::Type;

use super::block::BlockStmt;

pub enum Method {
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

impl Method {
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

impl Parse for Method {
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
            Ok(Method::Extern {
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
            Ok(Method::Default {
                span: Span::range(r#fun.span(), block.span()),
                ident,
                params,
                return_ty,
                block,
            })
        }
    }
}

impl DisplayTree for Method {
    fn display(&self, layer: usize) {
        match self {
            Self::Default {
                ident,
                params,
                return_ty,
                block,
                ..
            } => {
                branch("Method", layer);
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
                branch("ExternMethod", layer);
                ident.display(layer + 1);
                params.display(layer + 1);
                if let Some(return_ty) = &return_ty {
                    return_ty.display(layer + 1);
                }
            }
        }
    }
}

impl Parse for Vec<Method> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::RightBrace {
            Ok(vec![])
        } else {
            let mut methods = vec![];
            methods.push(input.parse()?);
            while input.peek().kind() != TokenKind::Eof
                && input.peek().kind() != TokenKind::RightBrace
            {
                methods.push(input.parse()?);
            }
            Ok(methods)
        }
    }
}

impl DisplayTree for Vec<Method> {
    fn display(&self, layer: usize) {
        for method in self {
            branch("Methods", layer);
            method.display(layer + 1);
        }
    }
}

pub struct ImplStmt {
    span: Span,
    tr: Option<Ident>,
    ty: Type,
    methods: Vec<Method>,
}

span!(ImplStmt);

impl ImplStmt {
    pub fn tr(&self) -> Option<&Ident> {
        self.tr.as_ref()
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn methods(&self) -> &Vec<Method> {
        &self.methods
    }
}

impl Parse for ImplStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#impl: Impl = input.parse()?;

        let tr = if input.peek2().kind() == TokenKind::For {
            let tr: Ident = input.parse()?;
            let _for: For = input.parse()?;
            Some(tr)
        } else {
            None
        };

        let ty: Type = input.parse()?;
        let _left_brace: LeftBrace = input.parse()?;
        let methods: Vec<Method> = input.parse()?;
        let right_brace: RightBrace = input.parse()?;
        Ok(Self {
            span: Span::range(r#impl.span(), right_brace.span()),
            tr,
            ty,
            methods,
        })
    }
}

impl DisplayTree for ImplStmt {
    fn display(&self, layer: usize) {
        branch("ImplStmt", layer);
        if let Some(tr) = &self.tr {
            tr.display(layer + 1);
        }
        self.ty.display(layer + 1);
        self.methods.display(layer + 1);
    }
}
