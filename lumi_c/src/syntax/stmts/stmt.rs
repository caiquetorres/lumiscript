use lumi_lxr::token::TokenKind;

use crate::compile_error::CompileError;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;

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
    If(StmtIf),
    While(StmtWhile),
    For(StmtFor),
    Break(StmtBreak),
    Return(StmtReturn),
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

        while input.peek() != TokenKind::RightBrace && input.peek() != TokenKind::Eof {
            stmts.push(input.parse::<Stmt>()?);
        }

        Ok(stmts)
    }
}

impl Parse for Vec<Box<Stmt>> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        let mut stmts = vec![];

        while input.peek() != TokenKind::RightBrace && input.peek() != TokenKind::Eof {
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
        TokenKind::LeftBrace => Ok(Stmt::Block(input.parse()?)),
        TokenKind::Let => Ok(Stmt::Let(input.parse()?)),
        TokenKind::Println => Ok(Stmt::Print(input.parse()?)),
        TokenKind::Static | TokenKind::Extern | TokenKind::Fun => Ok(Stmt::Fun(input.parse()?)),
        TokenKind::Impl => Ok(Stmt::Impl(input.parse()?)),
        TokenKind::Trait => Ok(Stmt::Trait(input.parse()?)),
        TokenKind::Class => Ok(Stmt::Class(input.parse()?)),
        TokenKind::Return => Ok(Stmt::Return(input.parse()?)),
        TokenKind::If => Ok(Stmt::If(input.parse()?)),
        TokenKind::While => Ok(Stmt::While(input.parse()?)),
        TokenKind::For => Ok(Stmt::For(input.parse()?)),
        TokenKind::Break => Ok(Stmt::Break(input.parse()?)),
        TokenKind::Continue => Ok(Stmt::Continue(input.parse()?)),
        TokenKind::Const => Ok(Stmt::Const(input.parse()?)),
        _ => Ok(Stmt::Expr(input.parse()?)),
    }
}
