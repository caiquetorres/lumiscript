use super::bytecode::Bytecode;
use super::constant::Constant;

#[derive(Debug, Clone)]
pub struct Chunk {
    buffer: Vec<Bytecode>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            constants: vec![],
        }
    }

    pub fn load_constant(&mut self, constant: Constant) {
        self.constants.push(constant);
        let pos = self.constants.len() - 1;
        self.write_op(Bytecode::LoadConstant(pos));
    }

    pub fn buffer(&self) -> &Vec<Bytecode> {
        &self.buffer
    }

    pub fn write_op(&mut self, bytecode: Bytecode) {
        self.buffer.push(bytecode);
    }

    pub fn constant(&self, i: usize) -> Option<Constant> {
        self.constants.get(i).cloned()
    }
}
