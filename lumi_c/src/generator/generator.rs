use crate::generator::chunk::Bytecode;
use crate::generator::chunk::Constant;
use crate::syntax::ast::Ast;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::exprs::lit::ExprLit;
use crate::syntax::stmts::stmt::Stmt;

use super::chunk::Chunk;

trait Generate {
    fn generate(&self, chunk: &mut Chunk);
}

impl Generate for Stmt {
    fn generate(&self, chunk: &mut Chunk) {
        match self {
            Stmt::Block(block) => {
                chunk.write_op(Bytecode::BeginScope);

                for stmt in block.stmts() {
                    stmt.generate(chunk);
                }

                chunk.write_op(Bytecode::EndScope);
            }
            Stmt::Let(stmt) => {
                stmt.expr().generate(chunk);
                let i = chunk.add_constant(Constant::Str(stmt.ident().name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::SetVar);
            }
            Stmt::Const(stmt) => {
                stmt.expr().generate(chunk);
                let i = chunk.add_constant(Constant::Str(stmt.ident().name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::SetVar);
            }
            Stmt::Expr(expr) => {
                expr.expr().generate(chunk);
                chunk.write_op(Bytecode::Pop);
            }
            Stmt::Class(class) => {
                let i = chunk.add_constant(Constant::Str(class.name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::DeclareClass);
            }
            _ => {}
        }
    }
}

impl Generate for Expr {
    fn generate(&self, chunk: &mut Chunk) {
        match self {
            Expr::Ident(ident) => {
                let i = chunk.add_constant(Constant::Str(ident.ident.name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::GetVar);
            }
            Expr::Lit(lit) => match lit {
                ExprLit::Num { span } => {
                    let value: f64 = span.source_text.parse().unwrap();
                    let i = chunk.add_constant(Constant::Float(value));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }
                ExprLit::Bool { span } => {
                    let value: bool = span.source_text.parse().unwrap();
                    let i = chunk.add_constant(Constant::Bool(value));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }
                _ => {}
            },
            Expr::Unary(unary) => {
                unary.expr.generate(chunk);
                match &unary.operator.name() as &str {
                    "+" => { /* do nothing */ }
                    "-" => chunk.write_op(Bytecode::Negate),
                    "!" => chunk.write_op(Bytecode::Not),
                    _ => unreachable!(),
                }
            }
            Expr::Paren(paren) => paren.expr.generate(chunk),
            Expr::Binary(bin) => {
                bin.left.generate(chunk);
                bin.right.generate(chunk);

                match &bin.operator.name() as &str {
                    "+" => chunk.write_op(Bytecode::Add),
                    "-" => chunk.write_op(Bytecode::Subtract),
                    "*" => chunk.write_op(Bytecode::Multiply),
                    "/" => chunk.write_op(Bytecode::Divide),
                    "==" => chunk.write_op(Bytecode::Equal),
                    "!=" => {
                        chunk.write_op(Bytecode::Equal);
                        chunk.write_op(Bytecode::Not);
                    }
                    ">" => chunk.write_op(Bytecode::Greater),
                    ">=" => {
                        chunk.write_op(Bytecode::Less);
                        chunk.write_op(Bytecode::Not);
                    }
                    "<" => chunk.write_op(Bytecode::Less),
                    "<=" => {
                        chunk.write_op(Bytecode::Greater);
                        chunk.write_op(Bytecode::Not);
                    }
                    _ => {}
                }
            }
            Expr::Class(class) => {
                if let Expr::Ident(ident) = class.class.as_ref() {
                    let i = chunk.add_constant(Constant::Str(ident.ident.name()));
                    chunk.write_op(Bytecode::LoadConstant(i));
                    chunk.write_op(Bytecode::InstantiateClass);
                }
            }
            _ => {}
        }
    }
}

pub struct Generator;

impl Generator {
    pub fn generate(ast: &Ast) -> Chunk {
        let mut chunk = Chunk::new();

        for stmt in ast.stmts() {
            stmt.generate(&mut chunk)
        }

        chunk
    }
}
