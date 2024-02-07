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
