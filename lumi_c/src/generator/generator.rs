use crate::generator::chunk::Bytecode;
use crate::generator::chunk::Constant;
use crate::syntax::ast::Ast;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::exprs::lit::ExprLit;
use crate::syntax::stmts::fun::StmtFun;
use crate::syntax::stmts::stmt::Stmt;

use super::chunk::Chunk;
use super::chunk::ObjectFunction;

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
            Stmt::Fun(func) => {
                match func {
                    StmtFun::Default { ident, block, .. } => {
                        let i = chunk.add_constant(Constant::Str(ident.name()));
                        chunk.write_op(Bytecode::LoadConstant(i));

                        let mut new_chunk = Chunk::new();

                        for stmt in block.stmts() {
                            stmt.generate(&mut new_chunk);
                        }
                        new_chunk.write_op(Bytecode::Return);

                        let mut func = Box::new(ObjectFunction {
                            arity: 0,
                            name: ident.name().clone(),
                            chunk: new_chunk,
                        });

                        let i = chunk
                            .add_constant(Constant::Func(func.as_mut() as *mut ObjectFunction));
                        chunk.write_op(Bytecode::LoadConstant(i));
                        chunk.write_op(Bytecode::DeclareFunc);

                        std::mem::forget(func); // avoid dropping the value when going out of scope.
                    }
                    StmtFun::Proto { .. } => {
                        // ignore, prototypes are only used for trait declarations and in the semantic analysis step.
                    }
                }
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
            Expr::Call(call) => {
                call.callee.generate(chunk);
                chunk.write_op(Bytecode::Call);
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
