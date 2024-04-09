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
    LoadConstant,
    ConvertConstant,
    BeginScope,
    EndScope,
    DeclareVariable,
    DeclareConst,
    DeclareClass,
    DeclareFunction,
    DeclareMethod,
    PrintLn,
    GetSymbol,
    SetVariable,
    GetProperty,
    SetProperty,
    CallFunction,
    Instantiate,
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
    Number(f64),
    String(String),
    Size(usize),
}

impl Constant {
    pub(crate) fn as_bool(&self) -> bool {
        match self {
            Self::Bool(value) => *value,
            _ => panic!("Cannot constant to bool"),
        }
    }

    pub(crate) fn as_number(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            _ => panic!("Cannot constant to number"),
        }
    }

    pub(crate) fn as_string(&self) -> String {
        match self {
            Self::String(value) => value.clone(),
            _ => panic!("Cannot constant to string"),
        }
    }

    pub(crate) fn as_size(&self) -> usize {
        match self {
            Self::Size(value) => *value,
            _ => panic!("Cannot constant to size"),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    instructions: Vec<u8>,
    constant_pool: Vec<Constant>,
    source_map: HashMap<usize, Span>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            constant_pool: vec![],
            source_map: HashMap::new(),
        }
    }

    pub(crate) fn span(&self, index: usize) -> &Span {
        self.source_map.get(&index).unwrap()
    }

    pub(crate) fn len(&self) -> usize {
        self.instructions.len()
    }

    pub(crate) fn push_constant(&mut self, constant: Constant, source: Span) {
        self.constant_pool.push(constant);
        let constant_index = self.constant_pool.len() - 1;
        self.push_instruction(Bytecode::LoadConstant, source);
        self.instructions.extend(&[
            ((constant_index >> 16) & 0xFF) as u8,
            ((constant_index >> 8) & 0xFF) as u8,
            (constant_index & 0xFF) as u8,
        ]);
    }

    pub(crate) fn push_instruction(&mut self, instruction: Bytecode, source: Span) {
        self.instructions.push(instruction as u8);
        self.source_map.insert(self.instructions.len() - 1, source);
    }

    pub(crate) fn instruction(&self, index: usize) -> Option<Bytecode> {
        self.instructions
            .get(index)
            .map(|instruction| Bytecode::from_byte(*instruction))
    }

    pub(crate) fn constant(&self, index: usize) -> Option<&Constant> {
        let constant_index = usize::from(self.instructions[index + 1]) << 16
            | usize::from(self.instructions[index + 2]) << 8
            | usize::from(self.instructions[index + 3]);
        self.constant_pool.get(constant_index)
    }

    pub(crate) fn constant_mut(&mut self, index: usize) -> Option<&mut Constant> {
        let constant_index = usize::from(self.instructions[index + 1]) << 16
            | usize::from(self.instructions[index + 2]) << 8
            | usize::from(self.instructions[index + 3]);
        self.constant_pool.get_mut(constant_index)
    }
}
