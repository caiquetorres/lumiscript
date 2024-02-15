use lumi_lxr::token::TokenKind;

use crate::display_tree::DisplayTree;
use crate::parse::Parse;
use crate::parser::{ParseError, ParseStream};

use self::block::BlockStmt;
use self::class::ClassStmt;
use self::expr::ExprStmt;
use self::println::PrintlnStmt;
use self::r#const::ConstStmt;
use self::r#for::ForStmt;
use self::r#if::IfStmt;
use self::r#impl::ImplStmt;
use self::r#let::LetStmt;
use self::r#trait::TraitStmt;
use self::r#while::WhileStmt;

pub mod block;
pub mod class;
pub mod r#const;
pub mod expr;
pub mod r#for;
pub mod r#if;
pub mod r#impl;
pub mod r#let;
pub mod println;
pub mod r#trait;
pub mod r#while;

pub enum Stmt {
    Println(PrintlnStmt),
    Block(BlockStmt),
    Let(LetStmt),
    Const(ConstStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Class(ClassStmt),
    Trait(TraitStmt),
    Impl(ImplStmt),
    Expr(ExprStmt),
}

impl Parse for Stmt {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        match input.peek().kind() {
            TokenKind::LeftBrace => Ok(Stmt::Block(input.parse()?)),
            TokenKind::Println => Ok(Stmt::Println(input.parse()?)),
            TokenKind::If => Ok(Stmt::If(input.parse()?)),
            TokenKind::While => Ok(Stmt::While(input.parse()?)),
            TokenKind::Let => Ok(Stmt::Let(input.parse()?)),
            TokenKind::Const => Ok(Stmt::Const(input.parse()?)),
            TokenKind::For => Ok(Stmt::For(input.parse()?)),
            TokenKind::Class => Ok(Stmt::Class(input.parse()?)),
            TokenKind::Trait => Ok(Stmt::Trait(input.parse()?)),
            TokenKind::Impl => Ok(Stmt::Impl(input.parse()?)),
            _ => Ok(Stmt::Expr(input.parse()?)),
        }
    }
}

impl DisplayTree for Stmt {
    fn display(&self, layer: usize) {
        match self {
            Stmt::Block(block) => block.display(layer),
            Stmt::If(r#if) => r#if.display(layer),
            Stmt::While(r#while) => r#while.display(layer),
            Stmt::Println(println) => println.display(layer),
            Stmt::Let(r#let) => r#let.display(layer),
            Stmt::Const(r#const) => r#const.display(layer),
            Stmt::For(r#for) => r#for.display(layer),
            Stmt::Class(class) => class.display(layer),
            Stmt::Trait(r#trait) => r#trait.display(layer),
            Stmt::Impl(r#impl) => r#impl.display(layer),
            Stmt::Expr(expr) => expr.display(layer),
        }
    }
}

impl Parse for Vec<Stmt> {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        let mut stmts = vec![];
        while input.peek().kind() != TokenKind::Eof {
            stmts.push(input.parse()?);
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
