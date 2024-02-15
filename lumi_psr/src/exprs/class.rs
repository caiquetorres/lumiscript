use lumi_lxr::span::Span;
use lumi_lxr::token::TokenKind;

use crate::display_tree::{branch, DisplayTree};
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};
use crate::symbols::{Colon, Ident, LeftBrace, RightBrace};

use super::Expr;

#[derive(Debug)]
pub struct Field {
    span: Span,
    ident: Ident,
    value: Option<Expr>,
}

impl Field {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn value(&self) -> Option<&Expr> {
        self.value.as_ref()
    }
}

impl Parse for Field {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let ident: Ident = input.parse()?;
        if input.peek().kind() == TokenKind::Colon {
            input.parse::<Colon>()?;
            let value: Expr = input.parse()?;
            Ok(Self {
                span: Span::range(ident.span(), value.span()),
                ident,
                value: Some(value),
            })
        } else {
            Ok(Self {
                span: Span::range(ident.span(), ident.span()),
                ident,
                value: None,
            })
        }
    }
}

impl DisplayTree for Field {
    fn display(&self, layer: usize) {
        branch("Field", layer);
        self.ident.display(layer + 1);
        if let Some(value) = &self.value {
            value.display(layer + 1);
        }
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
        if !self.is_empty() {
            branch("Fields", layer);
            for field in self {
                field.display(layer + 1);
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassExpr {
    span: Span,
    cls: Box<Expr>,
    fields: Vec<Field>,
}

impl ClassExpr {
    pub(crate) fn new(
        cls: Expr,
        _left_brace: LeftBrace,
        fields: Vec<Field>,
        right_brace: RightBrace,
    ) -> Self {
        Self {
            span: Span::range(cls.span(), right_brace.span()),
            cls: Box::new(cls),
            fields,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl DisplayTree for ClassExpr {
    fn display(&self, layer: usize) {
        branch("ClassExpr", layer);
        self.cls.display(layer + 1);
        self.fields.display(layer + 1);
    }
}
