use crate::compile_error::CompileError;
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
use super::r#break::StmtBreak;
use super::r#const::StmtConst;
use super::r#continue::StmtContinue;
use super::r#for::StmtFor;
use super::r#if::StmtIf;
use super::r#impl::StmtImpl;
use super::r#let::StmtLet;
use super::r#return::StmtReturn;
use super::r#trait::StmtTrait;
use super::r#while::StmtWhile;

pub enum Stmt {
    Let(StmtLet),
    Const(StmtConst),
    Print(StmtPrint),
    Block(StmtBlock),
    Fun(StmtFun),
    Expr(StmtExpr),
    Impl(StmtImpl),
    Trait(StmtTrait),
    Class(StmtClass),
    Return(StmtReturn),
    If(StmtIf),
    While(StmtWhile),
    For(StmtFor),
    Break(StmtBreak),
    Continue(StmtContinue),
}

impl Parse for Stmt {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        ambiguous_stmt(input)
    }
}

impl Parse for Box<Stmt> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        ambiguous_stmt(input).map(Box::new)
    }
}

impl Parse for Vec<Stmt> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        let mut stmts = vec![];

        while input.peek() != token!('}') && input.peek() != token!(eof) {
            stmts.push(input.parse::<Stmt>()?);
        }

        Ok(stmts)
    }
}

impl Parse for Vec<Box<Stmt>> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        let mut stmts = vec![];

        while input.peek() != token!('}') && input.peek() != token!(eof) {
            stmts.push(Box::new(input.parse::<Stmt>()?));
        }

        Ok(stmts)
    }
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
            Self::If(r#if) => r#if.display(layer),
            Self::While(r#while) => r#while.display(layer),
            Self::For(r#for) => r#for.display(layer),
            Self::Break(r#break) => r#break.display(layer),
            Self::Continue(r#continue) => r#continue.display(layer),
            Self::Const(r#const) => r#const.display(layer),
        }
    }
}

fn ambiguous_stmt(input: &mut ParseStream) -> Result<Stmt, CompileError> {
    match input.peek() {
        token!('{') => Ok(Stmt::Block(input.parse()?)),
        token!(let) => Ok(Stmt::Let(input.parse()?)),
        token!(println) => Ok(Stmt::Print(input.parse()?)),
        token!(static) | token!(extern) | token!(fun) => Ok(Stmt::Fun(input.parse()?)),
        token!(impl) => Ok(Stmt::Impl(input.parse()?)),
        token!(trait) => Ok(Stmt::Trait(input.parse()?)),
        token!(class) => Ok(Stmt::Class(input.parse()?)),
        token!(return) => Ok(Stmt::Return(input.parse()?)),
        token!(if) => Ok(Stmt::If(input.parse()?)),
        token!(while) => Ok(Stmt::While(input.parse()?)),
        token!(for) => Ok(Stmt::For(input.parse()?)),
        token!(break) => Ok(Stmt::Break(input.parse()?)),
        token!(continue) => Ok(Stmt::Continue(input.parse()?)),
        token!(const) => Ok(Stmt::Const(input.parse()?)),
        _ => Ok(Stmt::Expr(input.parse()?)),
    }
}
