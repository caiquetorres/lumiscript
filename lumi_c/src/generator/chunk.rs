use super::bytecode::Bytecode;
use super::obj::Obj;

#[derive(Debug, Clone)]
pub struct Chunk {
    buffer: Vec<Bytecode>,
    objects: Vec<Obj>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            objects: vec![],
        }
    }

    pub fn buffer(&self) -> &Vec<Bytecode> {
        &self.buffer
    }

    pub fn write_op(&mut self, bytecode: Bytecode) {
        self.buffer.push(bytecode);
    }

    pub fn add_constant(&mut self, constant: Obj) -> usize {
        self.objects.push(constant);
        self.objects.len() - 1
    }

    pub fn object(&self, i: usize) -> Option<Obj> {
        self.objects.get(i).cloned()
    }
}
