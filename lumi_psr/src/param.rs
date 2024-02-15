use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Colon, Ident};
use crate::ty::Type;

#[derive(Debug)]
pub struct Param {
    span: Span,
    ident: Ident,
    ty: Type,
}

span!(Param);

impl Parse for Param {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let ident: Ident = input.parse()?;
        let _colon: Colon = input.parse()?;
        let ty: Type = input.parse()?;
        Ok(Self {
            span: Span::range(ident.span(), ty.span()),
            ident,
            ty,
        })
    }
}

impl DisplayTree for Param {
    fn display(&self, layer: usize) {
        branch("Param", layer);
        self.ident.display(layer + 1);
        self.ty.display(layer + 1);
    }
}

impl Parse for Vec<Param> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::RightParen {
            Ok(vec![])
        } else {
            let mut params = vec![];
            params.push(input.parse()?);
            while input.peek().kind() == TokenKind::Comma {
                input.expect(TokenKind::Comma)?;
                params.push(input.parse()?);
            }
            Ok(params)
        }
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
