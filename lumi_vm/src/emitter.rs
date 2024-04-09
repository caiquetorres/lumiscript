use lumi_psr::{
    ast::Ast,
    exprs::{lit::LitExpr, Expr},
    stmts::{fun::FunStmt, r#impl::Method, Stmt},
};

use crate::chunk::{Bytecode, Chunk, Constant};

pub(crate) trait Emitter {
    fn emit(&self, chunk: &mut Chunk);
}

pub struct BytecodeEmitter;

impl BytecodeEmitter {
    pub fn emit(ast: &Ast) -> Chunk {
        let mut chunk = Chunk::new();
        ast.emit(&mut chunk);
        chunk
    }
}

impl Emitter for Ast {
    fn emit(&self, chunk: &mut Chunk) {
        for stmt in self.stmts() {
            stmt.emit(chunk);
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
                chunk.push_instruction(Bytecode::PrintLn, println.expr().span().clone());
            }
            Self::Block(block) => {
                chunk.push_instruction(Bytecode::BeginScope, block.span().clone());
                for stmt in block.stmts() {
                    stmt.emit(chunk);
                }
                chunk.push_instruction(Bytecode::EndScope, block.span().clone());
            }
            Self::Let(r#let) => {
                r#let.expr().emit(chunk);
                chunk.push_constant(
                    Constant::String(r#let.ident().source_text()),
                    r#let.ident().span().clone(),
                );
                chunk.push_instruction(Bytecode::DeclareVariable, r#let.expr().span().clone());
            }
            Self::Const(r#const) => {
                r#const.expr().emit(chunk);
                chunk.push_constant(
                    Constant::String(r#const.ident().source_text()),
                    r#const.ident().span().clone(),
                );
                chunk.push_instruction(Bytecode::DeclareVariable, r#const.expr().span().clone());
            }
            Self::Class(class) => {
                chunk.push_constant(
                    Constant::String(class.ident().span().source_text()),
                    class.ident().span().clone(),
                );
                chunk.push_instruction(Bytecode::DeclareClass, class.ident().span().clone());
            }
            Self::Return(r#return) => {
                if let Some(expr) = r#return.expr() {
                    expr.emit(chunk);
                } else {
                    chunk.push_constant(Constant::Nil, r#return.span().clone());
                    chunk.push_instruction(Bytecode::ConvertConstant, r#return.span().clone());
                }
                chunk.push_instruction(Bytecode::Return, r#return.span().clone());
            }
            Self::Fun(fun) => {
                if let FunStmt::Default {
                    ident, block, span, ..
                } = fun
                {
                    let function_end = chunk.len();
                    chunk.push_constant(Constant::Size(usize::MAX), span.clone());
                    let function_start = chunk.len();
                    chunk.push_constant(Constant::Size(usize::MAX), span.clone());
                    for param in fun.params().iter().rev() {
                        chunk.push_constant(
                            Constant::String(param.ident().span().source_text()),
                            param.ident().span().clone(),
                        );
                    }
                    chunk.push_constant(Constant::Size(fun.params().len()), span.clone());
                    chunk.push_constant(
                        Constant::String(ident.span().source_text()),
                        ident.span().clone(),
                    );
                    chunk.push_instruction(Bytecode::DeclareFunction, span.clone());
                    let start = chunk.len();
                    chunk.push_instruction(Bytecode::BeginScope, block.span().clone());
                    for stmt in block.stmts() {
                        stmt.emit(chunk);
                    }
                    chunk.push_constant(Constant::Nil, span.clone());
                    chunk.push_instruction(Bytecode::ConvertConstant, span.clone());
                    chunk.push_instruction(Bytecode::Return, span.clone());
                    let end = chunk.len();
                    if let Some(constant) = chunk.constant_mut(function_start) {
                        *constant = Constant::Size(start);
                    }
                    if let Some(constant) = chunk.constant_mut(function_end) {
                        *constant = Constant::Size(end);
                    }
                }
            }
            Self::Impl(r#impl) => {
                for method in r#impl.methods() {
                    chunk.push_constant(
                        Constant::String(r#impl.ty().ident().source_text()),
                        r#impl.ty().ident().span().clone(),
                    );
                    if let Method::Default {
                        ident, block, span, ..
                    } = method
                    {
                        let function_end = chunk.len();
                        chunk.push_constant(Constant::Size(usize::MAX), span.clone());
                        let function_start = chunk.len();
                        chunk.push_constant(Constant::Size(usize::MAX), span.clone());
                        for param in method.params().iter().rev() {
                            chunk.push_constant(
                                Constant::String(param.ident().span().source_text()),
                                param.ident().span().clone(),
                            );
                        }
                        chunk.push_constant(Constant::Size(method.params().len()), span.clone());
                        chunk.push_constant(
                            Constant::String(ident.span().source_text()),
                            ident.span().clone(),
                        );
                        chunk.push_instruction(Bytecode::DeclareMethod, r#impl.ty().span().clone());
                        let start = chunk.len();
                        chunk.push_instruction(Bytecode::BeginScope, block.span().clone());
                        for stmt in block.stmts() {
                            stmt.emit(chunk);
                        }
                        chunk.push_constant(Constant::Nil, span.clone());
                        chunk.push_instruction(Bytecode::ConvertConstant, span.clone());
                        chunk.push_instruction(Bytecode::Return, span.clone());
                        let end = chunk.len();
                        if let Some(constant) = chunk.constant_mut(function_start) {
                            *constant = Constant::Size(start);
                        }
                        if let Some(constant) = chunk.constant_mut(function_end) {
                            *constant = Constant::Size(end);
                        }
                    }
                }
            }
            Self::If(r#if) => {
                r#if.cond().emit(chunk);
                let then_jump = chunk.len();
                chunk.push_constant(Constant::Size(usize::MAX), r#if.span().clone());
                chunk.push_instruction(Bytecode::JumpIfFalse, r#if.span().clone());
                let start = chunk.len();
                chunk.push_instruction(Bytecode::BeginScope, r#if.span().clone());
                for stmt in r#if.stmts() {
                    stmt.emit(chunk);
                }
                chunk.push_instruction(Bytecode::EndScope, r#if.span().clone());
                let end = chunk.len();
                let offset = end - start;
                if let Some(constant) = chunk.constant_mut(then_jump) {
                    *constant = Constant::Size(offset);
                }
            }
            Self::Trait(_) => {}
            _ => todo!(),
        }
    }
}

impl Emitter for Expr {
    fn emit(&self, chunk: &mut Chunk) {
        match self {
            Self::Ident(ident) => {
                let ident_name = ident.ident().source_text();
                chunk.push_constant(Constant::String(ident_name), ident.ident().span().clone());
                chunk.push_instruction(Bytecode::GetSymbol, ident.span().clone());
            }
            Self::Lit(lit) => match lit {
                LitExpr::Num { span } => {
                    let num = span.source_text().parse::<f64>().unwrap();
                    chunk.push_constant(Constant::Number(num), span.clone());
                    chunk.push_instruction(Bytecode::ConvertConstant, span.clone());
                }
                LitExpr::Bool { span } => {
                    let num = span.source_text().parse::<bool>().unwrap();
                    chunk.push_constant(Constant::Bool(num), span.clone());
                    chunk.push_instruction(Bytecode::ConvertConstant, span.clone());
                }
                LitExpr::Nil { span } => {
                    chunk.push_constant(Constant::Nil, span.clone());
                    chunk.push_instruction(Bytecode::ConvertConstant, span.clone());
                }
            },
            Self::Get(get) => {
                get.expr().emit(chunk);
                chunk.push_constant(
                    Constant::String(get.ident().source_text()),
                    get.ident().span().clone(),
                );
                chunk.push_instruction(Bytecode::GetProperty, get.ident().span().clone());
            }
            Self::Call(call) => {
                call.callee().emit(chunk);
                for arg in call.args().iter().rev() {
                    arg.emit(chunk);
                }
                chunk.push_constant(Constant::Size(call.args().len()), call.span().clone());
                chunk.push_instruction(Bytecode::CallFunction, call.callee().span().clone());
            }
            Self::Paren(paren) => {
                paren.expr().emit(chunk);
            }
            Self::Unary(unary) => {
                unary.expr().emit(chunk);
                let op = &unary.op().source_text()[..];
                match op {
                    "-" => chunk.push_instruction(Bytecode::Negate, unary.span().clone()),
                    "!" => chunk.push_instruction(Bytecode::Not, unary.span().clone()),
                    _ => panic!("Operator not implemented yet"),
                }
            }
            Self::Binary(binary) => {
                binary.left().emit(chunk);
                binary.right().emit(chunk);
                let op = &binary.op().source_text()[..];
                match op {
                    "+" => chunk.push_instruction(Bytecode::Add, binary.span().clone()),
                    "-" => chunk.push_instruction(Bytecode::Subtract, binary.span().clone()),
                    "*" => chunk.push_instruction(Bytecode::Multiply, binary.span().clone()),
                    "/" => chunk.push_instruction(Bytecode::Divide, binary.span().clone()),
                    "==" => chunk.push_instruction(Bytecode::Equals, binary.span().clone()),
                    "!=" => {
                        chunk.push_instruction(Bytecode::Equals, binary.span().clone());
                        chunk.push_instruction(Bytecode::Not, binary.span().clone());
                    }
                    "=" => {
                        binary.right().emit(chunk);
                        let mut expr = binary.left();
                        while let Expr::Paren(paren) = expr {
                            expr = paren.expr();
                        }
                        if let Expr::Ident(ident) = expr {
                            chunk.push_constant(
                                Constant::String(ident.ident().span().source_text()),
                                ident.ident().span().clone(),
                            );
                            chunk.push_instruction(Bytecode::SetVariable, binary.span().clone());
                        } else if let Expr::Get(get) = expr {
                            chunk.push_constant(
                                Constant::String(get.ident().span().source_text()),
                                get.ident().span().clone(),
                            );
                            get.expr().emit(chunk);
                            chunk.push_instruction(Bytecode::SetProperty, binary.span().clone());
                        }
                    }
                    _ => todo!(),
                }
            }
            Self::Class(class) => {
                class.cls().emit(chunk);
                for field in class.fields().iter().rev() {
                    if let Some(value) = field.value() {
                        value.emit(chunk);
                    } else {
                        let ident_name = field.ident().source_text();
                        chunk.push_constant(
                            Constant::String(ident_name),
                            class.cls().span().clone(),
                        );
                        chunk.push_instruction(Bytecode::GetSymbol, class.cls().span().clone());
                    }
                    let field_name = field.ident().source_text();
                    chunk.push_constant(Constant::String(field_name), field.ident().span().clone());
                }
                chunk.push_constant(
                    Constant::Size(class.fields().len()),
                    class.cls().span().clone(),
                );
                chunk.push_instruction(Bytecode::Instantiate, class.cls().span().clone());
            }
        }
    }
}
