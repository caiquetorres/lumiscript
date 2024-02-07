use crate::syntax::ast::Ast;
use crate::syntax::exprs::expr::Expr;
use crate::syntax::exprs::lit::ExprLit;
use crate::syntax::stmts::fun::StmtFun;
use crate::syntax::stmts::stmt::Stmt;

use super::bytecode::Bytecode;
use super::chunk::Chunk;
use super::obj::{Obj, ObjFunc};

trait Generate {
    fn generate(&self, chunk: &mut Chunk);
}

impl Generate for Stmt {
    fn generate(&self, chunk: &mut Chunk) {
        match self {
            Stmt::Print(print) => {
                print.expr().generate(chunk);
                chunk.write_op(Bytecode::Println);
            }
            Stmt::Block(block) => {
                chunk.write_op(Bytecode::BeginScope);

                for stmt in block.stmts() {
                    stmt.generate(chunk);
                }

                chunk.write_op(Bytecode::EndScope);
            }
            Stmt::Fun(func) => {
                match func {
                    StmtFun::Default {
                        ident,
                        params,
                        block,
                        ..
                    } => {
                        let mut new_chunk = Chunk::new();
                        new_chunk.write_op(Bytecode::BeginScope);

                        for stmt in block.stmts() {
                            stmt.generate(&mut new_chunk);
                        }

                        new_chunk.write_op(Bytecode::EndScope);

                        let mut func = Box::new(ObjFunc {
                            params: params.iter().map(|param| param.ident().name()).collect(),
                            name: ident.name().clone(),
                            chunk: new_chunk,
                        });

                        let i = chunk.add_constant(Obj::Func(func.as_mut() as *mut ObjFunc));
                        chunk.write_op(Bytecode::LoadConstant(i));

                        let i = chunk.add_constant(Obj::Str(ident.name()));
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
                let i = chunk.add_constant(Obj::Str(stmt.ident().name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::SetVar);
            }
            Stmt::Const(stmt) => {
                stmt.expr().generate(chunk);
                let i = chunk.add_constant(Obj::Str(stmt.ident().name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::SetVar);
            }
            Stmt::Expr(expr) => {
                expr.expr().generate(chunk);
                chunk.write_op(Bytecode::Pop);
            }
            Stmt::Return(rt) => {
                if let Some(expr) = rt.expr() {
                    expr.generate(chunk);
                } else {
                    let i = chunk.add_constant(Obj::Nil);
                    chunk.write_op(Bytecode::LoadConstant(i));
                }

                chunk.write_op(Bytecode::Return);
            }
            Stmt::Class(class) => {
                let i = chunk.add_constant(Obj::Str(class.name()));
                chunk.write_op(Bytecode::LoadConstant(i));

                let i = chunk.add_constant(Obj::Float(class.fields().len() as f64));
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
                let i = chunk.add_constant(Obj::Str(ident.ident.name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::GetVar);
            }
            Expr::Get(get) => {
                get.expr.generate(chunk);
                let i = chunk.add_constant(Obj::Str(get.ident.name()));
                chunk.write_op(Bytecode::LoadConstant(i));
                chunk.write_op(Bytecode::GetProp);
            }
            Expr::Assign(assign) => {
                assign.right.generate(chunk);

                match assign.left.as_ref() {
                    Expr::Ident(ident) => {
                        let i = chunk.add_constant(Obj::Str(ident.ident.name()));
                        chunk.write_op(Bytecode::LoadConstant(i));
                        chunk.write_op(Bytecode::SetVar);
                    }
                    Expr::Get(get) => {
                        let i = chunk.add_constant(Obj::Str(get.ident.name()));
                        chunk.write_op(Bytecode::LoadConstant(i));
                        get.expr.generate(chunk);
                        chunk.write_op(Bytecode::SetProp);
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Lit(lit) => match lit {
                ExprLit::Nil { .. } => {
                    let i = chunk.add_constant(Obj::Nil);
                    chunk.write_op(Bytecode::LoadConstant(i));
                }
                ExprLit::Num { span } => {
                    let value: f64 = span.source_text.parse().unwrap();
                    let i = chunk.add_constant(Obj::Float(value));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }
                ExprLit::Bool { span } => {
                    let value: bool = span.source_text.parse().unwrap();
                    let i = chunk.add_constant(Obj::Bool(value));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }
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
                for arg in call.args.iter().rev() {
                    arg.generate(chunk);
                }

                let i = chunk.add_constant(Obj::Float(call.args.len() as f64));
                chunk.write_op(Bytecode::LoadConstant(i));

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
                for field_init in class.field_inits.iter().rev() {
                    let name = field_init.ident().name();

                    if let Some(init) = field_init.init() {
                        init.expr().generate(chunk);
                    } else {
                        let i = chunk.add_constant(Obj::Str(name.clone()));
                        chunk.write_op(Bytecode::LoadConstant(i));
                        chunk.write_op(Bytecode::GetVar);
                    }

                    let i = chunk.add_constant(Obj::Str(name));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }

                if let Expr::Ident(ident) = class.class.as_ref() {
                    let i = chunk.add_constant(Obj::Str(ident.ident.name()));
                    chunk.write_op(Bytecode::LoadConstant(i));
                }

                chunk.write_op(Bytecode::InstantiateClass);
            }
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
