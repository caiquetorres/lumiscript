use std::collections::HashMap;

use compiler::generator::chunk::Bytecode;
use compiler::generator::chunk::Chunk;
use compiler::generator::chunk::Constant;
use compiler::generator::chunk::ObjectClass;
use compiler::generator::chunk::ObjectInstance;

pub struct VirtualMachine;

#[derive(Debug)]
struct Scope {
    parent: Option<Box<Scope>>,
    data: HashMap<String, Constant>,
}

impl Scope {
    pub fn new(parent: Box<Scope>) -> Self {
        Self {
            parent: Some(parent),
            data: HashMap::new(),
        }
    }

    pub fn global() -> Self {
        Self {
            parent: None,
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ident: &str, constant: Constant) {
        self.data.insert(ident.to_owned(), constant);
    }

    pub fn get(&self, ident: &str) -> Option<Constant> {
        if let Some(constant) = self.data.get(ident) {
            Some(constant.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(ident)
        } else {
            None
        }
    }
}

impl VirtualMachine {
    pub fn run(chunk: Chunk) {
        let global_scope = Scope::global();
        let mut current_scope = global_scope;

        let mut stack = Vec::new();

        for instruction in chunk.buffer().clone() {
            match instruction {
                Bytecode::BeginScope => {
                    current_scope = Scope::new(Box::new(current_scope));
                }
                Bytecode::EndScope => {
                    if let Some(parent) = current_scope.parent {
                        current_scope = *parent;
                    }
                }
                Bytecode::LoadConstant(i) => stack.push(chunk.get_constant(i)),
                Bytecode::GetVar => {
                    let var = stack.pop().unwrap();
                    if let Constant::Str(ident) = var {
                        let constant = current_scope.get(&ident).unwrap();
                        stack.push(constant.clone());
                    }
                }
                Bytecode::SetVar => {
                    let var = stack.pop().unwrap();
                    if let Constant::Str(ident) = var {
                        current_scope.insert(&ident, stack.pop().unwrap());
                    }
                }
                Bytecode::DeclareClass => {
                    let constant = stack.pop().unwrap();
                    if let Constant::Str(name) = constant {
                        let mut ptr = Box::new(ObjectClass { name: name.clone() });
                        let class = Constant::Class(ptr.as_mut() as *mut ObjectClass);

                        current_scope.insert(&name, class);
                        std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                    }
                }
                Bytecode::InstantiateClass => {
                    let constant = stack.pop().unwrap();
                    if let Constant::Str(name) = constant {
                        let c = current_scope.get(&name).unwrap();
                        if let Constant::Class(class) = c {
                            let mut ptr = Box::new(ObjectInstance { class });
                            let instance = Constant::Instance(ptr.as_mut() as *mut ObjectInstance);

                            stack.push(instance);

                            std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                        }
                    }
                }
                Bytecode::GetConst => {
                    let var = stack.pop().unwrap();
                    if let Constant::Str(ident) = var {
                        let constant = current_scope.get(&ident).unwrap();
                        stack.push(constant.clone());
                    }
                }
                Bytecode::SetConst => {
                    let var = stack.pop().unwrap();
                    if let Constant::Str(ident) = var {
                        current_scope.data.insert(ident, stack.pop().unwrap());
                    }
                }
                Bytecode::Add => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 + val2;
                        stack.push(Constant::Float(result));
                    }
                }
                Bytecode::Subtract => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 - val2;
                        stack.push(Constant::Float(result));
                    }
                }
                Bytecode::Multiply => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 * val2;
                        stack.push(Constant::Float(result));
                    }
                }
                Bytecode::Divide => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 / val2;
                        stack.push(Constant::Float(result));
                    }
                }
                Bytecode::Equal => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (&operand1, &operand2) {
                        let result = val1 == val2;
                        stack.push(Constant::Bool(result));
                    } else if let (Constant::Bool(val1), Constant::Bool(val2)) =
                        (&operand1, &operand2)
                    {
                        let result = val1 == val2;
                        stack.push(Constant::Bool(result));
                    }
                }
                Bytecode::Greater => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 > val2;
                        stack.push(Constant::Bool(result));
                    }
                }
                Bytecode::Less => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Constant::Float(val1), Constant::Float(val2)) = (operand1, operand2) {
                        let result = val1 < val2;
                        stack.push(Constant::Bool(result));
                    }
                }
                Bytecode::Negate => {
                    let num = stack.pop().unwrap();

                    if let Constant::Float(val) = num {
                        let result = -val;
                        stack.push(Constant::Float(result));
                    }
                }
                Bytecode::Not => {
                    let num = stack.pop().unwrap();

                    if let Constant::Bool(val) = num {
                        let result = !val;
                        stack.push(Constant::Bool(result));
                    }
                }
                Bytecode::Return => {}
                Bytecode::Pop => {
                    if let Some(result) = stack.pop() {
                        println!("{:?}", result);
                    }
                }
                _ => {}
            }
        }

        // println!("{:?}", stack);
        // println!("{:?}", current_scope);
    }
}
