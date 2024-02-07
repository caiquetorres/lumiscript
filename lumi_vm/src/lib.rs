pub mod call_frame;
pub mod scope;

use std::collections::HashMap;

use call_frame::CallFrame;
use call_frame::CallFrameStack;
use compiler::generator::bytecode::Bytecode;
use compiler::generator::obj::Obj;
use compiler::generator::obj::ObjCls;
use compiler::generator::obj::ObjFunc;
use compiler::generator::obj::ObjInst;
use scope::ScopeStack;

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn run(root_fun: ObjFunc) {
        let mut scope_stack = ScopeStack::new();
        let mut frames = CallFrameStack::new();
        let mut stack: Vec<Obj> = Vec::new();

        let mut root_frame = Box::new(root_fun);
        frames.push(CallFrame::new(root_frame.as_mut() as *mut ObjFunc, vec![]));

        std::mem::forget(root_frame); // avoid dropping the value when going out of scope.

        let mut should_update_counter = true;

        while let Some(instruction) = frames.current().and_then(|frame| frame.peek()) {
            match instruction {
                Bytecode::BeginScope => scope_stack.push(),
                Bytecode::EndScope => scope_stack.pop(),
                Bytecode::DeclareFunc => {
                    let func = stack.pop().unwrap();
                    let func_name = stack.pop().unwrap().as_str();
                    scope_stack.insert(&func_name, func);
                }
                Bytecode::Call => {
                    let func = stack.pop().map(|obj| obj.as_function());

                    if let Some(func) = func {
                        frames.push(CallFrame::new(func, vec![]));
                        should_update_counter = false;
                    }
                }
                Bytecode::LoadConstant(i) => {
                    let obj = frames.current().and_then(|frame| frame.object(i));

                    if let Some(obj) = obj {
                        stack.push(obj);
                    }
                }
                Bytecode::GetVar => {
                    let ident = stack.pop().unwrap().as_str();
                    let constant = scope_stack.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetVar => {
                    let ident = stack.pop().unwrap().as_str();
                    scope_stack.insert(&ident, stack.pop().unwrap());
                }
                Bytecode::DeclareClass => {
                    let fields_count = stack.pop().unwrap().as_float();
                    let class_name = stack.pop().unwrap().as_str();

                    let mut ptr = Box::new(ObjCls {
                        name: class_name.clone(),
                        fields_count: fields_count as u32,
                    });

                    let class = Obj::Class(ptr.as_mut() as *mut ObjCls);
                    scope_stack.insert(&class_name, class);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetProp => {
                    let prop = stack.pop().unwrap().as_str();
                    let inst = stack.pop().unwrap().as_instance();

                    unsafe {
                        let inst = &*inst;
                        let c = inst.prop(&prop).unwrap().clone();
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
                    let class = scope_stack.get(&class_name).unwrap().as_class();
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

                    let mut ptr = Box::new(ObjInst::new(class, props));
                    let instance = Obj::Instance(ptr.as_mut() as *mut ObjInst);

                    stack.push(instance);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetConst => {
                    let ident = stack.pop().unwrap().as_str();
                    let constant = scope_stack.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetConst => {
                    let ident = stack.pop().unwrap().as_str();
                    scope_stack.insert(&ident, stack.pop().unwrap());
                }
                Bytecode::Add => {
                    let operand2 = stack.pop().map(|obj| obj.as_float());
                    let operand1 = stack.pop().map(|obj| obj.as_float());

                    if let (Some(val1), Some(val2)) = (operand1, operand2) {
                        let result = val1 + val2;
                        stack.push(Obj::Float(result));
                    }
                }
                Bytecode::Subtract => {
                    let operand2 = stack.pop().map(|obj| obj.as_float());
                    let operand1 = stack.pop().map(|obj| obj.as_float());

                    if let (Some(val1), Some(val2)) = (operand1, operand2) {
                        let result = val1 - val2;
                        stack.push(Obj::Float(result));
                    }
                }
                Bytecode::Multiply => {
                    let operand2 = stack.pop().map(|obj| obj.as_float());
                    let operand1 = stack.pop().map(|obj| obj.as_float());

                    if let (Some(val1), Some(val2)) = (operand1, operand2) {
                        let result = val1 * val2;
                        stack.push(Obj::Float(result));
                    }
                }
                Bytecode::Divide => {
                    let operand2 = stack.pop().map(|obj| obj.as_float());
                    let operand1 = stack.pop().map(|obj| obj.as_float());

                    if let (Some(val1), Some(val2)) = (operand1, operand2) {
                        let result = val1 / val2;
                        stack.push(Obj::Float(result));
                    }
                }
                Bytecode::Equal => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Obj::Float(val1), Obj::Float(val2)) = (&operand1, &operand2) {
                        let result = val1 == val2;
                        stack.push(Obj::Bool(result));
                    } else if let (Obj::Bool(val1), Obj::Bool(val2)) = (&operand1, &operand2) {
                        let result = val1 == val2;
                        stack.push(Obj::Bool(result));
                    }
                }
                Bytecode::Greater => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Obj::Float(val1), Obj::Float(val2)) = (operand1, operand2) {
                        let result = val1 > val2;
                        stack.push(Obj::Bool(result));
                    }
                }
                Bytecode::Less => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    if let (Obj::Float(val1), Obj::Float(val2)) = (operand1, operand2) {
                        let result = val1 < val2;
                        stack.push(Obj::Bool(result));
                    }
                }
                Bytecode::Negate => {
                    let num = stack.pop().unwrap();

                    if let Obj::Float(val) = num {
                        let result = -val;
                        stack.push(Obj::Float(result));
                    }
                }
                Bytecode::Not => {
                    let num = stack.pop().unwrap();

                    if let Obj::Bool(val) = num {
                        let result = !val;
                        stack.push(Obj::Bool(result));
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
                frames.next();
            } else {
                should_update_counter = true;
            }
        }

        // println!("output: {:?}", stack);
        // println!("{:?}", current_scope);
    }
}