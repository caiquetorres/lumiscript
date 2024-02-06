use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::brace::LeftBrace;
use crate::syntax::symbols::brace::RightBrace;
use crate::syntax::symbols::class::Class;
use crate::syntax::symbols::ident::Ident;
use crate::token;

pub struct Field {
    ident: Ident,
    ty: Type,
}

impl Parse for Field {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Field {
            ident: input.parse()?,
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
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!('}') {
            Ok(vec![])
        } else {
            let mut fields = vec![];
            fields.push(input.parse()?);

            while input.peek() != token!('}') {
                input.expect(token!(,))?;
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
        self.ident.name()
    }
}

impl Parse for StmtClass {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
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
