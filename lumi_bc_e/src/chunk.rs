use std::collections::HashMap;

use lumi_lxr::span::Span;

macro_rules! define_bytecodes {
    ($($name:ident),*) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy)]
        pub enum Bytecode {
            $($name,)*
        }

        impl Bytecode {
            pub fn from_byte(byte: u8) -> Bytecode {
                match byte {
                    $(bytecode if bytecode == Bytecode::$name as u8 => Bytecode::$name,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}

define_bytecodes!(
    LoadConst,
    ConvertConst,
    BeginScope,
    EndScope,
    DeclareVar,
    DeclareConst,
    DeclareClass,
    DeclareFun,
    DeclareMethod,
    Println,
    GetSymbol,
    SetVar,
    GetProp,
    SetProp,
    CallFun,
    InstantiateClass,
    Implement,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Not,
    Equals,
    Greater,
    Less,
    JumpIfFalse,
    Jump,
    Else,
    While,
    Return,
    Pop
);

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Bool(bool),
    Float(f64),
    Size(usize),
    Str(String),
}

impl Constant {
    pub fn index_size() -> usize {
        3
    }
}

#[derive(Debug)]
pub struct Chunk {
    buffer: Vec<u8>,
    constants: Vec<Constant>,
    source_map: HashMap<usize, Span>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            constants: vec![],
            source_map: HashMap::new(),
        }
    }

    pub fn size(&self) -> f32 {
        (std::mem::size_of::<Vec<u8>>()
            + self.buffer.capacity() * std::mem::size_of::<u8>()
            + std::mem::size_of::<Vec<Constant>>()
            + self.buffer.capacity() * std::mem::size_of::<Constant>()) as f32
            / 1024.0
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn add_bytecode(&mut self, bytecode: Bytecode) {
        self.buffer.push(bytecode as u8);
    }

    pub fn add_bytecode_2(&mut self, bytecode: Bytecode, source: Span) {
        self.buffer.push(bytecode as u8);
        self.source_map.insert(self.buffer.len() - 1, source);
    }

    pub fn span(&self, pos: usize) -> Option<Span> {
        self.source_map.get(&pos).map(|value| value.clone())
    }

    pub fn bytecode(&self, index: usize) -> Option<Bytecode> {
        self.buffer
            .get(index)
            .map(|byte| Bytecode::from_byte(*byte))
    }

    pub fn add_constant(&mut self, constant: Constant) {
        self.constants.push(constant);
        self.add_bytecode(Bytecode::LoadConst);
        let index = self.constants.len() - 1;
        self.buffer.extend(&[
            ((index >> 16) & 0xFF) as u8,
            ((index >> 8) & 0xFF) as u8,
            (index & 0xFF) as u8,
        ]);
    }

    pub fn constant(&self, index: usize) -> Option<&Constant> {
        let index = usize::from(self.buffer[index + 1]) << 16
            | usize::from(self.buffer[index + 2]) << 8
            | usize::from(self.buffer[index + 3]);
        self.constants.get(index)
    }

    pub fn constant_mut(&mut self, index: usize) -> Option<&mut Constant> {
        let index = usize::from(self.buffer[index + 1]) << 16
            | usize::from(self.buffer[index + 2]) << 8
            | usize::from(self.buffer[index + 3]);
        self.constants.get_mut(index)
    }
}
