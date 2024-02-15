use lumi_lxr::span;
use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::ident;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Class, Colon, Ident, LeftBrace, RightBrace};
use crate::ty::Type;

#[derive(Debug)]
pub struct Field {
    span: Span,
    ident: Ident,
    ty: Type,
}

span!(Field);

impl Parse for Field {
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

impl DisplayTree for Field {
    fn display(&self, layer: usize) {
        branch("Field", layer);
        self.ident.display(layer + 1);
        self.ty.display(layer + 1);
    }
}

impl Parse for Vec<Field> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        if input.peek().kind() == TokenKind::RightBrace {
            Ok(vec![])
        } else {
            let mut fields = vec![];
            fields.push(input.parse()?);
            while input.peek().kind() != TokenKind::RightBrace {
                input.expect(TokenKind::Comma)?;
                fields.push(input.parse()?);
            }
            Ok(fields)
        }
    }
}

impl DisplayTree for Vec<Field> {
    fn display(&self, layer: usize) {
        branch("Fields", layer);
        for field in self {
            field.display(layer + 1);
        }
    }
}

pub struct ClassStmt {
    span: Span,
    ident: Ident,
    fields: Vec<Field>,
}

span!(ClassStmt);
ident!(ClassStmt);

impl ClassStmt {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl Parse for ClassStmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let class: Class = input.parse()?;
        let ident: Ident = input.parse()?;
        let _left_brace: LeftBrace = input.parse()?;
        let fields: Vec<Field> = input.parse()?;
        let right_brace: RightBrace = input.parse()?;
        Ok(Self {
            span: Span::range(class.span(), right_brace.span()),
            ident,
            fields,
        })
    }
}

impl DisplayTree for ClassStmt {
    fn display(&self, layer: usize) {
        branch("ClassStmt", layer);
        self.ident.display(layer + 1);
        self.fields.display(layer + 1);
    }
}
