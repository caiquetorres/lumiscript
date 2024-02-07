use std::collections::HashMap;

use compiler::generator::chunk::Bytecode;
use compiler::generator::chunk::CallFrame;
use compiler::generator::chunk::CallFrameStack;
use compiler::generator::chunk::Object;
use compiler::generator::chunk::ObjectClass;
use compiler::generator::chunk::ObjectFunction;
use compiler::generator::chunk::ObjectInstance;

pub struct VirtualMachine;

#[derive(Debug)]
struct Scope {
    parent: Option<Box<Scope>>,
    data: HashMap<String, Object>,
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

    pub fn insert(&mut self, ident: &str, constant: Object) {
        self.data.insert(ident.to_owned(), constant);
    }

    pub fn get(&self, ident: &str) -> Option<Object> {
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
    pub fn run(root_fun: ObjectFunction) {
        let global_scope = Scope::global();
        let mut current_scope = global_scope;

        let mut frames = CallFrameStack::new();
        let mut stack: Vec<Object> = Vec::new();

        let mut root_frame = Box::new(root_fun);

        frames.add(CallFrame::new(
            root_frame.as_mut() as *mut ObjectFunction,
            vec![],
        ));

        std::mem::forget(root_frame);

        let mut should_update_counter = true;

        while let Some(instruction) = frames.get_instruction() {
            match instruction {
                Bytecode::BeginScope => {
                    current_scope = Scope::new(Box::new(current_scope));
                }
                Bytecode::EndScope => {
                    if let Some(parent) = current_scope.parent {
                        current_scope = *parent;
                    }
                }
                Bytecode::DeclareFunc => {
                    let func = stack.pop().unwrap();
                    let func_name = stack.pop().unwrap().as_str();
                    current_scope.insert(&func_name, func);
                }
                Bytecode::Call => {
                    let func = stack.pop().unwrap();

                    if let Object::Func(func) = func {
                        frames.add(CallFrame::new(func, vec![]));
                        should_update_counter = false;
                    }
                }
                Bytecode::LoadConstant(i) => {
                    stack.push(frames.current().unwrap().function().chunk.get_constant(i))
                }
                Bytecode::GetVar => {
                    let ident = stack.pop().unwrap().as_str();
                    let constant = current_scope.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetVar => {
                    let ident = stack.pop().unwrap().as_str();
                    current_scope.insert(&ident, stack.pop().unwrap());
                }
                Bytecode::DeclareClass => {
                    let fields_count = stack.pop().unwrap().as_float();
                    let class_name = stack.pop().unwrap().as_str();

                    let mut ptr = Box::new(ObjectClass {
                        name: class_name.clone(),
                        fields_count: fields_count as u32,
                    });

                    let class = Object::Class(ptr.as_mut() as *mut ObjectClass);
                    current_scope.insert(&class_name, class);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetProp => {
                    let prop = stack.pop().unwrap().as_str();
                    let inst = stack.pop().unwrap().as_instance();

                    unsafe {
                        let inst = &*inst;
                        let c = inst.get_prop(&prop).unwrap().clone();
                        stack.push(c);
                    }
                }
                Bytecode::SetProp => {
                    let inst = stack.pop().unwrap().as_instance();
                    let prop = stack.pop().unwrap().as_str();
                    let value = stack.pop().unwrap();

                    unsafe {
                        let inst = &mut *inst;
                        inst.set_prop(&prop, value);
                    }
                }
                Bytecode::InstantiateClass => {
                    let class_name = stack.pop().unwrap().as_str();
                    let class = current_scope.get(&class_name).unwrap().as_class();
                    let mut props = HashMap::new();

                    unsafe {
                        let obj_class = &*class;

                        // REVIEW: I don't like this strategy, we're creating the fields based on their count.
                        for _ in 0..obj_class.fields_count {
                            let field_name = stack.pop().unwrap().as_str();
                            let field_value = stack.pop().unwrap();
                            props.insert(field_name, field_value);
                        }
                    }

                    let mut ptr = Box::new(ObjectInstance::new(class, props));
                    let instance = Object::Instance(ptr.as_mut() as *mut ObjectInstance);

                    stack.push(instance);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetConst => {
                    let ident = stack.pop().unwrap().as_str();
                    let constant = current_scope.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetConst => {
                    let ident = stack.pop().unwrap().as_str();
                    current_scope.data.insert(ident, stack.pop().unwrap());
                }
                Bytecode::Add => {
                    let val1 = stack.pop().unwrap().as_float();
                    let val2 = stack.pop().unwrap().as_float();
                    let result = val1 + val2;

                    stack.push(Object::Float(result));
                }
                Bytecode::Subtract => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (operand1, operand2) {
                        let result = val1 - val2;
                        stack.push(Object::Float(result));
                    }
                }
                Bytecode::Multiply => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (operand1, operand2) {
                        let result = val1 * val2;
                        stack.push(Object::Float(result));
                    }
                }
                Bytecode::Divide => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (operand1, operand2) {
                        let result = val1 / val2;
                        stack.push(Object::Float(result));
                    }
                }
                Bytecode::Equal => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (&operand1, &operand2) {
                        let result = val1 == val2;
                        stack.push(Object::Bool(result));
                    } else if let (Object::Bool(val1), Object::Bool(val2)) = (&operand1, &operand2)
                    {
                        let result = val1 == val2;
                        stack.push(Object::Bool(result));
                    }
                }
                Bytecode::Greater => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (operand1, operand2) {
                        let result = val1 > val2;
                        stack.push(Object::Bool(result));
                    }
                }
                Bytecode::Less => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Object::Float(val1), Object::Float(val2)) = (operand1, operand2) {
                        let result = val1 < val2;
                        stack.push(Object::Bool(result));
                    }
                }
                Bytecode::Negate => {
                    let num = stack.pop().unwrap();

                    if let Object::Float(val) = num {
                        let result = -val;
                        stack.push(Object::Float(result));
                    }
                }
                Bytecode::Not => {
                    let num = stack.pop().unwrap();

                    if let Object::Bool(val) = num {
                        let result = !val;
                        stack.push(Object::Bool(result));
                    }
                }
                Bytecode::Return => {
                    frames.pop();
                }
                Bytecode::Pop => {
                    if let Some(result) = stack.pop() {
                        println!("{:?}", result);
                        // if let Object::Instance(inst) = result {
                        //     unsafe {
                        //         println!("{:?}", &*inst);
                        //     }
                        // }
                    }
                }
            }

            if should_update_counter {
                frames.next_instruction();
            } else {
                should_update_counter = true;
            }
        }

        // println!("output: {:?}", stack);
        // println!("{:?}", current_scope);
    }
}
