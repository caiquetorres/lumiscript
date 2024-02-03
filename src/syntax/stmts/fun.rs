use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::r#type::Type;
use crate::syntax::symbols::arrow::Arrow;
use crate::syntax::symbols::brace::LeftBrace;
use crate::syntax::symbols::brace::RightBrace;
use crate::syntax::symbols::fun::Fun;
use crate::syntax::symbols::ident::Ident;
use crate::syntax::symbols::paren::LeftParen;
use crate::syntax::symbols::paren::RightParen;
use crate::syntax::symbols::semicolon::Semicolon;
use crate::token;

use super::stmt::Stmt;

pub struct Param {
    ident: Ident,
    ty: Type,
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
        self.ty.display(layer + 1);
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
    ident: Ident,
    nullable: bool,
}

impl Parse for ReturnType {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(ReturnType {
            _arrow: input.parse()?,
            ident: input.parse()?,
            nullable: {
                if input.peek() == token!(?) {
                    input.next();
                    true
                } else {
                    false
                }
            },
        })
    }
}

impl Parse for Option<ReturnType> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        if input.peek() == token!(->) {
            Ok(Some(input.parse()?))
        } else {
            Ok(None)
        }
    }
}

impl DisplayTree for ReturnType {
    fn display(&self, layer: usize) {
        branch(
            &format!(
                "Return: {}{}",
                self.ident.name(),
                if self.nullable { "?" } else { "" }
            ),
            layer,
        );
    }
}

pub struct StmtFun {
    _fun: Fun,
    ident: Ident,
    _left_paren: LeftParen,
    params: Vec<Param>,
    _right_paren: RightParen,
    return_ty: Option<ReturnType>,
    _left_brace: LeftBrace,
    stmts: Vec<Box<Stmt>>,
    _right_brace: RightBrace,
}

impl Parse for StmtFun {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtFun {
            _fun: input.parse()?,
            ident: input.parse()?,
            _left_paren: input.parse()?,
            params: input.parse()?,
            _right_paren: input.parse()?,
            return_ty: input.parse()?,
            _left_brace: input.parse()?,
            stmts: input.parse()?,
            _right_brace: input.parse()?,
        })
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
        branch("FunStmt", layer);
        self.ident.display(layer + 1);
        self.params.display(layer + 1);

        if let Some(return_ty) = &self.return_ty {
            return_ty.display(layer + 1);
        }

        if !self.stmts.is_empty() {
            branch("Body", layer + 1);
            self.stmts.display(layer + 2);
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

pub struct StmtFunProto {
    _fun: Fun,
    ident: Ident,
    _left_paren: LeftParen,
    params: Vec<Param>,
    _right_paren: RightParen,
    return_ty: Option<ReturnType>,
    _semicolon: Semicolon,
}

impl Parse for StmtFunProto {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        Ok(StmtFunProto {
            _fun: input.parse()?,
            ident: input.parse()?,
            _left_paren: input.parse()?,
            params: input.parse()?,
            _right_paren: input.parse()?,
            return_ty: input.parse()?,
            _semicolon: input.parse()?,
        })
    }
}

impl Parse for Vec<StmtFunProto> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut functions = vec![];

        while input.peek() != token!('}') {
            functions.push(input.parse()?)
        }

        Ok(functions)
    }
}

impl DisplayTree for StmtFunProto {
    fn display(&self, layer: usize) {
        branch("FunStmt", layer);
        self.ident.display(layer + 1);
        self.params.display(layer + 1);

        if let Some(return_ty) = &self.return_ty {
            return_ty.display(layer + 1);
        }
    }
}

impl DisplayTree for Vec<StmtFunProto> {
    fn display(&self, layer: usize) {
        for fun in self {
            fun.display(layer);
        }
    }
}
