use crate::generator::chunk::{Constant, OpCode};
use crate::syntax::ast::Ast;

use super::chunk::Chunk;

pub struct Generator;

impl Generator {
    pub fn generate(_ast: &Ast) -> Chunk {
        let mut chunk = Chunk::new();

        let c = chunk.add_constant(Constant::Value(1.2));

        chunk.write_op(OpCode::Constant);
        chunk.write_op_as_byte(c);
        chunk.write_op(OpCode::Return);

        println!("{:#?}", chunk);
        println!("{:#?}", chunk.disassemble());

        chunk
    }
}
