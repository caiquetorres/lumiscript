use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::brace::LeftBrace;
use crate::syntax::symbols::brace::RightBrace;
use crate::syntax::symbols::colon::Colon;
use crate::syntax::symbols::ident::Ident;
use crate::token;

use super::expr::Expr;

pub struct Init {
    _colon: Colon,
    expr: Expr,
}

impl Init {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for Init {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(Init {
            _colon: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl Parse for Option<Init> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(:) {
            Ok(Some(Init {
                _colon: input.parse()?,
                expr: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct FieldInit {
    ident: Ident,
    init: Option<Init>,
}

impl FieldInit {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn init(&self) -> &Option<Init> {
        &self.init
    }
}

impl Parse for FieldInit {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(FieldInit {
            ident: input.parse()?,
            init: input.parse()?,
        })
    }
}

impl DisplayTree for FieldInit {
    fn display(&self, layer: usize) {
        branch("Field", layer);
        self.ident.display(layer + 1);

        if let Some(init) = &self.init {
            init.expr.display(layer + 1);
        }
    }
}

impl Parse for Vec<FieldInit> {
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

impl DisplayTree for Vec<FieldInit> {
    fn display(&self, layer: usize) {
        if !self.is_empty() {
            branch("Fields", layer);
            for field in self {
                field.display(layer + 1);
            }
        }
    }
}

pub struct ExprClass {
    pub class: Box<Expr>,
    pub left_brace: LeftBrace,
    pub field_inits: Vec<FieldInit>,
    pub right_brace: RightBrace,
}

impl DisplayTree for ExprClass {
    fn display(&self, layer: usize) {
        branch("ClassExpr", layer);
        self.class.display(layer + 1);
        self.field_inits.display(layer + 1);
    }
}
