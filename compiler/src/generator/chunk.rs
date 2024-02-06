use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Bytecode {
    BeginScope,
    EndScope,
    LoadConstant(usize), // Not the best solution
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Not,
    Equal,
    Greater,
    Less,
    Pop,
    SetVar,
    GetVar,
    SetConst,
    GetConst,
    Return,
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Bool(bool),
    Float(f64),
    Str(String),
    Obj(*mut Object),
}

pub enum Object {}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

    pub fn buffer(&self) -> &Vec<Bytecode> {
        &self.buffer
    }

    pub fn write_op(&mut self, op_code: Bytecode) {
        self.buffer.push(op_code);
    }

    pub fn add_constant(&mut self, constant: Constant) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, i: usize) -> Constant {
        self.constants[i].clone()
    }
}
