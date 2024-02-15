use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::ident;
use crate::param::Param;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{
    Arrow, Fun, Ident, LeftBrace, LeftParen, RightBrace, RightParen, Semicolon, Trait,
};
use crate::ty::Type;

#[derive(Debug)]
pub struct ProtoMethod {
    span: Span,
    ident: Ident,
    params: Vec<Param>,
    return_ty: Option<Type>,
}

span!(ProtoMethod);
ident!(ProtoMethod);

impl ProtoMethod {
    pub fn params(&self) -> &Vec<Param> {
        &self.params
    }

    pub fn return_ty(&self) -> Option<&Type> {
        self.return_ty.as_ref()
    }
}

impl Parse for ProtoMethod {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
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
        let semicolon: Semicolon = input.parse()?;
        Ok(Self {
            span: Span::range(r#fun.span(), semicolon.span()),
            ident,
            params,
            return_ty,
        })
    }
}

impl DisplayTree for ProtoMethod {
    fn display(&self, layer: usize) {
        branch("ProtoMethod", layer);
        self.ident.display(layer + 1);
        self.params.display(layer + 1);
        if let Some(return_ty) = &self.return_ty {
            return_ty.display(layer + 1);
        }
    }
}

impl Parse for Vec<ProtoMethod> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::RightBrace {
            Ok(vec![])
        } else {
            let mut protos = vec![];
            protos.push(input.parse()?);
            while input.peek().kind() != TokenKind::Eof
                && input.peek().kind() != TokenKind::RightBrace
            {
                protos.push(input.parse()?);
            }
            Ok(protos)
        }
    }
}

impl DisplayTree for Vec<ProtoMethod> {
    fn display(&self, layer: usize) {
        branch("Protos", layer);
        for proto in self {
            proto.display(layer + 1);
        }
    }
}

#[derive(Debug)]
pub struct TraitStmt {
    span: Span,
    ident: Ident,
    protos: Vec<ProtoMethod>,
}

span!(TraitStmt);
ident!(TraitStmt);

impl TraitStmt {
    pub fn protos(&self) -> &Vec<ProtoMethod> {
        &self.protos
    }
}

impl Parse for TraitStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let r#trait: Trait = input.parse()?;
        let ident: Ident = input.parse()?;
        let _left_brace: LeftBrace = input.parse()?;
        let protos: Vec<ProtoMethod> = input.parse()?;
        let right_brace: RightBrace = input.parse()?;
        Ok(Self {
            span: Span::range(r#trait.span(), right_brace.span()),
            ident,
            protos,
        })
    }
}

impl DisplayTree for TraitStmt {
    fn display(&self, layer: usize) {
        branch("TraitStmt", layer);
        self.ident.display(layer + 1);
        self.protos.display(layer + 1);
    }
}
