use lumi_psr::ast::Ast;

use crate::chunk::Chunk;
use crate::emitter::Emitter;

impl Emitter for Ast {
    fn emit(&self, chunk: &mut Chunk) {
        for stmt in self.stmts() {
            stmt.emit(chunk);
        }
    }
}
