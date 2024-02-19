use lumi_psr::stmts::fun::FunStmt;
use lumi_psr::stmts::r#impl::Method;
use lumi_psr::stmts::Stmt;

use crate::chunk::{Bytecode, Chunk, Constant};
use crate::emitter::Emitter;

impl Emitter for Method {
    fn emit(&self, chunk: &mut Chunk) {
        if let Method::Default { block, .. } = self {
            let end = chunk.len();
            chunk.add_constant(Constant::Size(usize::MAX));
            let start = chunk.len();
            chunk.add_constant(Constant::Size(usize::MAX));
            for param in self.params().iter().rev() {
                chunk.add_constant(Constant::Str(param.ident().span().source_text()));
            }
            chunk.add_constant(Constant::Size(self.params().len()));
            chunk.add_constant(Constant::Str(self.ident().span().source_text()));
            chunk.add_bytecode(Bytecode::DeclareMethod); // Declare method?
            let s = chunk.len();
            chunk.add_bytecode(Bytecode::BeginScope);
            for stmt in block.stmts() {
                stmt.emit(chunk);
            }
            chunk.add_bytecode(Bytecode::Return);
            let e = chunk.len();
            if let Some(constant) = chunk.constant_mut(start) {
                *constant = Constant::Size(s);
            }
            if let Some(constant) = chunk.constant_mut(end) {
                *constant = Constant::Size(e);
            }
        }
    }
}

impl Emitter for FunStmt {
    fn emit(&self, chunk: &mut Chunk) {
        if let FunStmt::Default { block, .. } = self {
            let end = chunk.len();
            chunk.add_constant(Constant::Size(usize::MAX));
            let start = chunk.len();
            chunk.add_constant(Constant::Size(usize::MAX));
            for param in self.params().iter().rev() {
                chunk.add_constant(Constant::Str(param.ident().span().source_text()));
            }
            chunk.add_constant(Constant::Size(self.params().len()));
            chunk.add_constant(Constant::Str(self.ident().span().source_text()));
            chunk.add_bytecode(Bytecode::DeclareFun); // Declare method?
            let s = chunk.len();
            chunk.add_bytecode(Bytecode::BeginScope);
            for stmt in block.stmts() {
                stmt.emit(chunk);
            }
            chunk.add_bytecode(Bytecode::Return);
            let e = chunk.len();
            if let Some(constant) = chunk.constant_mut(start) {
                *constant = Constant::Size(s);
            }
            if let Some(constant) = chunk.constant_mut(end) {
                *constant = Constant::Size(e);
            }
        }
    }
}

impl Emitter for Stmt {
    fn emit(&self, chunk: &mut Chunk) {
        match self {
            Self::Expr(expr) => {
                expr.expr().emit(chunk);
            }
            Self::Println(println) => {
                println.expr().emit(chunk);
                chunk.add_bytecode(Bytecode::Println);
            }
            Self::Block(block) => {
                chunk.add_bytecode(Bytecode::BeginScope);
                for stmt in block.stmts() {
                    stmt.emit(chunk);
                }
                chunk.add_bytecode(Bytecode::EndScope);
            }
            Self::Let(r#let) => {
                r#let.expr().emit(chunk);
                chunk.add_constant(Constant::Str(r#let.ident().source_text()));
                chunk.add_bytecode(Bytecode::DeclareVar);
            }
            Self::Const(r#const) => {
                r#const.expr().emit(chunk);
                chunk.add_constant(Constant::Str(r#const.ident().source_text()));
                chunk.add_bytecode(Bytecode::DeclareVar);
            }

            Self::Class(class) => {
                chunk.add_constant(Constant::Str(class.ident().span().source_text()));
                chunk.add_bytecode(Bytecode::DeclareClass);
            }
            Self::Impl(r#impl) => {
                for method in r#impl.methods() {
                    chunk.add_constant(Constant::Str(r#impl.ty().ident().source_text()));
                    method.emit(chunk);
                }
            }
            Self::Fun(fun) => {
                fun.emit(chunk);
            }
            Self::Return(r#return) => {
                r#return.expr().emit(chunk);
                chunk.add_bytecode(Bytecode::Return);
            }
            Self::Trait(_) => {}
            _ => todo!(),
        }
    }
}
