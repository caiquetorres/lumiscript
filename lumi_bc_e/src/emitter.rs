use lumi_psr::ast::Ast;

use crate::chunk::Chunk;

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
