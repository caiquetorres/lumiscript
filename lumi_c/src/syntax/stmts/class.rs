use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::LeftBrace;
use crate::syntax::symbols::RightBrace;
use crate::syntax::symbols::Class;
use crate::syntax::symbols::Colon;
use crate::syntax::symbols::Ident;

pub struct Field {
    ident: Ident,
    _colon: Colon,
    ty: Type,
}

impl Parse for Field {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Field {
            ident: input.parse()?,
            _colon: input.parse()?,
            ty: input.parse()?,
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
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        if input.peek() == TokenKind::RightBrace {
            Ok(vec![])
        } else {
            let mut fields = vec![];
            fields.push(input.parse()?);

            while input.peek() != TokenKind::RightBrace {
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

pub struct StmtClass {
    _class: Class,
    ident: Ident,
    _left_brace: LeftBrace,
    fields: Vec<Field>,
    _right_brace: RightBrace,
}

impl StmtClass {
    pub fn name(&self) -> String {
        self.ident.source_text()
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl Parse for StmtClass {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(StmtClass {
            _class: input.parse()?,
            ident: input.parse()?,
            _left_brace: input.parse()?,
            fields: input.parse()?,
            _right_brace: input.parse()?,
        })
    }
}

impl DisplayTree for StmtClass {
    fn display(&self, layer: usize) {
        branch("ClassStmt", layer);
        self.ident.display(layer + 1);
        self.fields.display(layer + 1);
    }
}
