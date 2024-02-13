use crate::compile_error::CompileError;
use crate::scanner::token::TokenKind;
use crate::syntax::display_tree::branch;
use crate::syntax::display_tree::DisplayTree;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::parse::Parse;
use crate::syntax::parse::ParseStream;
use crate::syntax::symbols::r#else::Else;
use crate::syntax::symbols::r#if::If;
use crate::token;

use super::stmt::Stmt;

pub struct StmtElse {
    _else: Else,
    stmt: Box<Stmt>,
}

impl StmtElse {
    pub fn stmt(&self) -> &Stmt {
        self.stmt.as_ref()
    }
}

impl Parse for StmtElse {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Self {
            _else: input.parse()?,
            stmt: input.parse()?,
        })
    }
}

impl Parse for Option<StmtElse> {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Some(StmtElse {
            _else: input.parse()?,
            stmt: input.parse()?,
        }))
    }
}

impl DisplayTree for StmtElse {
    fn display(&self, layer: usize) {
        branch("ElseStmt", layer);
        self.stmt.display(layer + 1);
    }
}

pub struct StmtIf {
    _if: If,
    cond: Expr,
    stmt: Box<Stmt>,
    r#else: Option<StmtElse>,
}

impl StmtIf {
    pub fn cond(&self) -> &Expr {
        &self.cond
    }

    pub fn stmt(&self) -> &Stmt {
        &self.stmt.as_ref()
    }

    pub fn r#else(&self) -> Option<&StmtElse> {
        self.r#else.as_ref()
    }
}

impl Parse for StmtIf {
    fn parse(input: &mut ParseStream) -> Result<Self, CompileError> {
        Ok(Self {
            _if: input.parse()?,
            cond: Expr::parse_without_eager_brace(input)?,
            stmt: input.parse()?,
            r#else: {
                if input.peek() == token!(else) {
                    input.parse()?
                } else {
                    None
                }
            },
        })
    }
}

impl DisplayTree for StmtIf {
    fn display(&self, layer: usize) {
        branch("IfStmt", layer);
        branch("Condition", layer + 1);
        self.cond.display(layer + 2);
        self.stmt.display(layer + 1);

        if let Some(r#else) = &self.r#else {
            r#else.display(layer);
        }
    }
}
