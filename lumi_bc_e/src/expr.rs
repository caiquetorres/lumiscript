use lumi_psr::exprs::lit::LitExpr;
use lumi_psr::exprs::Expr;

use crate::chunk::{Bytecode, Chunk, Constant};
use crate::emitter::Emitter;

impl Emitter for Expr {
    fn emit(&self, chunk: &mut Chunk) {
        match self {
            Self::Ident(ident) => {
                let ident_name = ident.ident().source_text();
                chunk.add_constant(Constant::Str(ident_name));
                chunk.add_bytecode(Bytecode::GetSymbol);
            }
            Self::Lit(lit) => match lit {
                LitExpr::Num { span } => {
                    let num = span.source_text().parse::<f64>().unwrap();
                    chunk.add_constant(Constant::Float(num));
                    chunk.add_bytecode(Bytecode::ConvertConst);
                }
                LitExpr::Bool { span } => {
                    let num = span.source_text().parse::<bool>().unwrap();
                    chunk.add_constant(Constant::Bool(num));
                    chunk.add_bytecode(Bytecode::ConvertConst);
                }
                LitExpr::Nil { .. } => {
                    chunk.add_constant(Constant::Nil);
                    chunk.add_bytecode(Bytecode::ConvertConst);
                }
            },
            Self::Get(get) => {
                get.expr().emit(chunk);
                chunk.add_constant(Constant::Str(get.ident().source_text()));
                chunk.add_bytecode(Bytecode::GetProp);
            }
            Self::Call(call) => {
                call.callee().emit(chunk);
                for arg in call.args().iter().rev() {
                    arg.emit(chunk);
                }
                chunk.add_constant(Constant::Size(call.args().len()));
                chunk.add_bytecode(Bytecode::CallFun);
            }
            Self::Paren(paren) => {
                paren.expr().emit(chunk);
            }
            Self::Unary(unary) => {
                unary.expr().emit(chunk);
                let op = &unary.op().source_text()[..];
                if let "-" = op {
                    chunk.add_bytecode(Bytecode::Negate);
                } else if let "!" = op {
                    chunk.add_bytecode(Bytecode::Not);
                }
            }
            Self::Binary(binary) => {
                binary.left().emit(chunk);
                binary.right().emit(chunk);
                match &binary.op().source_text()[..] {
                    "+" => chunk.add_bytecode(Bytecode::Add),
                    "-" => chunk.add_bytecode(Bytecode::Subtract),
                    "*" => chunk.add_bytecode(Bytecode::Multiply),
                    "/" => chunk.add_bytecode(Bytecode::Divide),
                    "==" => chunk.add_bytecode(Bytecode::Equals),
                    "!=" => {
                        chunk.add_bytecode(Bytecode::Equals);
                        chunk.add_bytecode(Bytecode::Not);
                    }
                    "=" => {
                        binary.right().emit(chunk);
                        let mut expr = binary.left();
                        while let Expr::Paren(paren) = expr {
                            expr = paren.expr();
                        }
                        if let Expr::Ident(ident) = expr {
                            chunk.add_constant(Constant::Str(ident.ident().span().source_text()));
                            chunk.add_bytecode(Bytecode::SetVar);
                        } else if let Expr::Get(get) = expr {
                            chunk.add_constant(Constant::Str(get.ident().span().source_text()));
                            get.expr().emit(chunk);
                            chunk.add_bytecode(Bytecode::SetProp);
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
                        chunk.add_constant(Constant::Str(ident_name));
                        chunk.add_bytecode(Bytecode::GetSymbol);
                    }
                    let field_name = field.ident().source_text();
                    chunk.add_constant(Constant::Str(field_name));
                }
                chunk.add_constant(Constant::Size(class.fields().len()));
                chunk.add_bytecode(Bytecode::InstantiateClass);
            }
        }
    }
}
