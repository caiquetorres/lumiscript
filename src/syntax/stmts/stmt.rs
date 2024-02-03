use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::token;

use super::block::StmtBlock;
use super::class::StmtClass;
use super::expr::StmtExpr;
use super::fun::StmtFun;
use super::print::StmtPrint;
use super::r#impl::StmtImpl;
use super::r#let::StmtLet;
use super::r#return::StmtReturn;
use super::r#trait::StmtTrait;

pub enum Stmt {
    Let(StmtLet),
    Print(StmtPrint),
    Block(StmtBlock),
    Fun(StmtFun),
    Expr(StmtExpr),
    Impl(StmtImpl),
    Trait(StmtTrait),
    Class(StmtClass),
    Return(StmtReturn),
}

impl DisplayTree for Vec<Stmt> {
    fn display(&self, layer: usize) {
        for stmt in self {
            stmt.display(layer);
        }
    }
}

impl DisplayTree for Vec<Box<Stmt>> {
    fn display(&self, layer: usize) {
        for stmt in self {
            stmt.display(layer);
        }
    }
}

impl DisplayTree for Stmt {
    fn display(&self, layer: usize) {
        match self {
            Self::Let(r#let) => r#let.display(layer),
            Self::Print(print) => print.display(layer),
            Self::Block(block) => block.display(layer),
            Self::Expr(expr) => expr.display(layer),
            Self::Fun(fun) => fun.display(layer),
            Self::Impl(r#impl) => r#impl.display(layer),
            Self::Trait(tr) => tr.display(layer),
            Self::Class(class) => class.display(layer),
            Self::Return(rt) => rt.display(layer),
        }
    }
}

impl Parse for Stmt {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        ambiguous_stmt(input)
    }
}

impl Parse for Vec<Stmt> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut stmts = vec![];

        while input.peek() != token!('}') && input.peek() != token!(eof) {
            stmts.push(input.parse::<Stmt>()?);
        }

        Ok(stmts)
    }
}

impl Parse for Vec<Box<Stmt>> {
    fn parse(input: &mut ParseStream) -> Result<Self, String> {
        let mut stmts = vec![];

        while input.peek() != token!('}') && input.peek() != token!(eof) {
            stmts.push(Box::new(input.parse::<Stmt>()?));
        }

        Ok(stmts)
    }
}

fn ambiguous_stmt(input: &mut ParseStream) -> Result<Stmt, String> {
    match input.peek() {
        token!('{') => Ok(Stmt::Block(input.parse::<StmtBlock>()?)),
        token!(let) => Ok(Stmt::Let(input.parse::<StmtLet>()?)),
        token!(println) => Ok(Stmt::Print(input.parse::<StmtPrint>()?)),
        token!(fun) => Ok(Stmt::Fun(input.parse::<StmtFun>()?)),
        token!(impl) => Ok(Stmt::Impl(input.parse::<StmtImpl>()?)),
        token!(trait) => Ok(Stmt::Trait(input.parse::<StmtTrait>()?)),
        token!(class) => Ok(Stmt::Class(input.parse::<StmtClass>()?)),
        token!(return) => Ok(Stmt::Return(input.parse::<StmtReturn>()?)),
        _ => Ok(Stmt::Expr(input.parse::<StmtExpr>()?)),
    }
}
