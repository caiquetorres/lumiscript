use std::{collections::HashMap, fmt::Display};

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
    GetVar,
    SetVar,
    GetProp,
    SetProp,
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
pub enum Object {
    Nil,
    Bool(bool),
    Float(f64),
    Str(String),
    Instance(*mut ObjectInstance),
    Class(*mut ObjectClass),
    Func(*mut ObjectFunction),
}

impl Object {
    pub fn as_bool(&self) -> bool {
        if let Object::Bool(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_float(&self) -> f64 {
        if let Object::Float(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_str(&self) -> String {
        if let Object::Str(value) = self {
            value.clone()
        } else {
            panic!();
        }
    }

    pub fn as_instance(&self) -> *mut ObjectInstance {
        if let Object::Instance(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_class(&self) -> *mut ObjectClass {
        if let Object::Class(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_function(&self) -> *mut ObjectFunction {
        if let Object::Func(value) = self {
            *value
        } else {
            panic!();
        }
    }
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
    pub fields_count: u32,
}

#[derive(Debug)]
pub struct ObjectInstance {
    class: *mut ObjectClass,
    props: HashMap<String, Object>,
}

impl ObjectInstance {
    pub fn new(class: *mut ObjectClass, props: HashMap<String, Object>) -> Self {
        Self { class, props }
    }

    pub fn class_ptr(&self) -> *mut ObjectClass {
        *&self.class
    }

    pub fn get_prop(&self, prop_name: &str) -> Option<Object> {
        self.props.get(prop_name).map(|obj| obj.clone())
    }

    pub fn set_prop(&mut self, prop_name: &str, prop_value: Object) {
        self.props.insert(prop_name.to_owned(), prop_value);
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct CallFrame {
    ip: usize,
    function: *mut ObjectFunction,
    pub slots: Vec<Object>,
}

impl CallFrame {
    pub fn new(function: *mut ObjectFunction, slots: Vec<Object>) -> Self {
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
    constants: Vec<Object>,
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

    pub fn add_constant(&mut self, constant: Object) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, i: usize) -> Object {
        self.constants[i].clone()
    }
}
