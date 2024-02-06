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
    DeclareClass,
    InstantiateClass,
    DeclareFunc,
    Call,
    Return,
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TODO: We should rename this Constant enum to Object or Obj.
// TODO: We should add the Prim (primitive) variant.

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Bool(bool),
    Float(f64),
    Str(String),
    Instance(*mut ObjectInstance),
    Class(*mut ObjectClass),
    Func(*mut ObjectFunction),
}

#[derive(Debug)]
pub struct ObjectInstance {
    pub class: *mut ObjectClass,
}

#[derive(Debug)]
pub struct ObjectFunction {
    pub name: String,
    pub arity: u8,
    pub chunk: Chunk,
}

#[derive(Debug)]
pub struct ObjectClass {
    pub name: String,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct CallFrame {
    ip: usize,
    function: *mut ObjectFunction,
    pub slots: Vec<Constant>,
}

impl CallFrame {
    pub fn new(function: *mut ObjectFunction, slots: Vec<Constant>) -> Self {
        Self {
            ip: 0,
            function,
            slots,
        }
    }

    pub fn function(&self) -> &ObjectFunction {
        unsafe { &*self.function }
    }
}

#[derive(Debug)]
pub struct CallFrameStack {
    frames: Vec<CallFrame>,
}

impl CallFrameStack {
    pub fn new() -> Self {
        Self { frames: vec![] }
    }

    pub fn get_instruction(&self) -> Option<Bytecode> {
        self.current().and_then(|frame| {
            frame
                .function()
                .chunk
                .buffer()
                .get(frame.ip)
                .map(|bytecode| *bytecode)
        })
    }

    pub fn next_instruction(&mut self) {
        if let Some(current) = self.current_mut() {
            current.ip += 1;
        }
    }

    pub fn current(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    pub fn add(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
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
