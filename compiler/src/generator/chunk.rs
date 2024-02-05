use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            _ => unreachable!(),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Value(f64),
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Chunk {
    data: Vec<u8>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            constants: vec![],
            data: vec![],
        }
    }

    pub fn add_constant(&mut self, constant: Constant) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }

    pub fn write_op_as_byte(&mut self, op_code: u8) {
        self.data.push(op_code);
    }

    pub fn write_op(&mut self, op_code: OpCode) {
        self.data.push(op_code as u8);
    }

    pub fn disassemble(&self) -> Vec<String> {
        let mut i = 0;
        let mut instructions = vec![];

        while i < self.data.len() {
            let (str_rep, j) = self.disassemble_instruction(i);
            instructions.push(str_rep);
            i += j;
        }

        instructions
    }

    fn disassemble_instruction(&self, offset: usize) -> (String, usize) {
        match OpCode::from(self.data[offset]) {
            OpCode::Constant => {
                let pos = self.data[offset + 1];
                (
                    format!(
                        "{} {} {}",
                        OpCode::Constant,
                        pos,
                        self.constants[pos as usize]
                    ),
                    offset + 2,
                )
            }
            op => (format!("{}", op), offset + 1),
        }
    }
}
