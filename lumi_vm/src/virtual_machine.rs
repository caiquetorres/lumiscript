use std::collections::HashMap;
use std::time::Instant;

use compiler::generator::bytecode::Bytecode;
use compiler::generator::constant::Constant;

use crate::call_frame::CallFrame;
use crate::call_frame::CallFrameStack;
use crate::obj::Obj;
use crate::obj::ObjBoundMethod;
use crate::obj::ObjBoundMethodFunc;
use crate::obj::ObjClass;
use crate::obj::ObjFunc;
use crate::obj::ObjInst;
use crate::obj::ObjNativeFunc;
use crate::obj::ObjPrim;
use crate::obj::ObjPrimKind;
use crate::scope::ScopeStack;

trait RawPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<T> RawPtr<T> for Box<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut() as *mut T
    }
}

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn run(root_fun: ObjFunc) {
        let mut scope_stack = ScopeStack::new();
        let mut frames = CallFrameStack::new();
        let mut stack: Vec<Obj> = Vec::new();

        let mut root_frame = Box::new(root_fun);

        frames.push(CallFrame::new(
            root_frame.as_mut() as *mut ObjFunc,
            HashMap::new(),
        ));

        let mut should_update_counter = true;

        let mut num_class = ObjClass {
            name: "Num".to_owned(),
            fields_count: 0,
        };

        let mut bool_class = ObjClass {
            name: "Bool".to_owned(),
            fields_count: 0,
        };

        let mut nil_class = ObjClass {
            name: "Nil".to_owned(),
            fields_count: 0,
        };

        let num_class_ptr = &mut num_class as *mut ObjClass;
        let bool_class_ptr = &mut bool_class as *mut ObjClass;
        let nil_class_ptr = &mut nil_class as *mut ObjClass;

        scope_stack.insert("Num", Obj::Class(num_class_ptr));
        scope_stack.insert("Bool", Obj::Class(bool_class_ptr));
        scope_stack.insert("Nil", Obj::Class(nil_class_ptr));

        let start_time = Instant::now();

        let mut clock_native_func = Box::new(ObjNativeFunc {
            name: "clock".to_owned(),
            func: Box::new(move |_, _| -> Obj {
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;

                let mut prim =
                    Box::new(ObjPrim::num(num_class_ptr, elapsed_time.as_millis() as f64));
                let res = Obj::Prim(prim.as_mut_ptr());
                std::mem::forget(prim);
                res
            }),
        });

        scope_stack.insert(
            &clock_native_func.name.clone(),
            Obj::NativeFunc(clock_native_func.as_mut() as *mut ObjNativeFunc),
        );

        let mut nil_to_bool = Box::new(ObjNativeFunc {
            name: "toBool".to_owned(),
            func: Box::new(move |_, _| -> Obj {
                let mut prim = Box::new(ObjPrim::bool(bool_class_ptr, false));
                let res = Obj::Prim(prim.as_mut() as *mut ObjPrim);
                std::mem::forget(prim);
                res
            }),
        });

        scope_stack.set_method(
            nil_class_ptr,
            &nil_to_bool.name.clone(),
            Obj::NativeFunc(nil_to_bool.as_mut() as *mut ObjNativeFunc),
        );

        while let Some(instruction) = frames.current().and_then(|frame| frame.peek()) {
            match instruction {
                Bytecode::Lit => {
                    let constant = stack.pop().unwrap().as_const();

                    let mut prim = Box::new(match constant {
                        Constant::Num(num) => ObjPrim::num(num_class_ptr, num),
                        Constant::Nil => ObjPrim::nil(nil_class_ptr),
                        Constant::Bool(b) => ObjPrim::bool(bool_class_ptr, b),
                        _ => panic!("Cannot convert this type to a literal"),
                    });

                    stack.push(Obj::Prim(prim.as_mut() as *mut ObjPrim));
                    std::mem::forget(prim);
                }
                Bytecode::LoadConstant(i) => {
                    let obj = frames
                        .current()
                        .and_then(|frame| frame.constant(i))
                        .and_then(|c| Some(Obj::Constant(c)))
                        .unwrap();

                    stack.push(obj);
                }
                Bytecode::Println => {
                    let obj = stack.pop().unwrap();

                    match obj {
                        Obj::Prim(prim) => unsafe {
                            let prim = &*prim;
                            match prim.kind {
                                ObjPrimKind::Nil => println!("nil"),
                                ObjPrimKind::Num => println!("{}", prim.value),
                                ObjPrimKind::Bool => println!("{}", prim.value != 0.0),
                            }
                        },
                        Obj::Func(_) => println!("<func>"),
                        Obj::Class(class) => unsafe {
                            let class = &*class;
                            println!("<class {}>", class.name);
                        },
                        Obj::Inst(inst) => unsafe {
                            let inst = &*inst;
                            let class = &*inst.class_ptr();
                            println!("<instance {}>", class.name);
                        },
                        _ => {}
                    }
                }
                Bytecode::BeginScope => {
                    scope_stack.push();

                    if let Some(current) = frames.current() {
                        for (key, object) in current.slots() {
                            scope_stack.insert(key, object.clone());
                        }
                    }
                }
                Bytecode::EndScope => {
                    scope_stack.pop();
                }
                Bytecode::DeclareFunc => {
                    let func_name = stack.pop().expect("Empty stack").as_const().as_str();
                    let func_const = stack.pop().expect("Empty stack").as_const().as_function();

                    let mut func = Box::new(ObjFunc {
                        chunk: func_const.chunk().clone(),
                        name: func_const.name().clone(),
                        params: func_const.params().clone(),
                    });

                    scope_stack.insert(&func_name, Obj::Func(func.as_mut() as *mut ObjFunc));
                    std::mem::forget(func);
                }
                Bytecode::DeclareMethod => {
                    let class_name = stack.pop().expect("Empty stack").as_const().as_str();
                    let func_name = stack.pop().expect("Empty stack").as_const().as_str();
                    let func_const = stack.pop().expect("Empty stack").as_const().as_function();

                    let mut func = Box::new(ObjFunc {
                        chunk: func_const.chunk().clone(),
                        name: func_const.name().clone(),
                        params: func_const.params().clone(),
                    });

                    let class = scope_stack.get(&class_name).unwrap().as_class();
                    scope_stack.set_method(
                        class,
                        &func_name,
                        Obj::Func(func.as_mut() as *mut ObjFunc),
                    );

                    std::mem::forget(func);
                }
                Bytecode::Call => {
                    let func = stack.pop().unwrap();

                    if let Obj::BoundMethod(method) = func {
                        unsafe {
                            let method = &*method;
                            let arity = stack.pop().map(|obj| obj.as_const().as_num());

                            let mut args = vec![];

                            if let Some(arity) = arity {
                                for _ in 0..arity as u8 {
                                    let arg = stack.pop().unwrap();
                                    args.push(arg);
                                }
                            }

                            let this = method.this.clone();

                            let mut slots = HashMap::new();
                            slots.insert("this".to_owned(), this.clone());

                            match this {
                                Obj::Inst(inst) => {
                                    let inst = &*inst;
                                    slots.insert("This".to_owned(), Obj::Class(inst.class_ptr()));
                                }
                                Obj::Prim(prim) => {
                                    let prim = &*prim;
                                    slots.insert("This".to_owned(), Obj::Class(prim.class_ptr()));
                                }
                                _ => unreachable!(),
                            }

                            match method.func {
                                ObjBoundMethodFunc::Default(func_ptr) => {
                                    let func = &*func_ptr;
                                    for i in 0..func.params.len() {
                                        slots.insert(func.params[i].clone(), args[i].clone());
                                    }

                                    frames.push(CallFrame::new(func_ptr, slots));
                                    should_update_counter = false;
                                }
                                ObjBoundMethodFunc::Native(native_func_ptr) => {
                                    let func = &*native_func_ptr;
                                    let res = (func.func)(arity.unwrap() as usize, args);
                                    stack.push(res);
                                }
                            }
                        }
                    } else if let Obj::Func(func) = func {
                        let arity = stack.pop().map(|obj| obj.as_const().as_num());

                        let mut args = vec![];

                        if let Some(arity) = arity {
                            for _ in 0..arity as u8 {
                                let arg = stack.pop().unwrap();
                                args.push(arg);
                            }
                        }

                        let mut slots = HashMap::new();

                        unsafe {
                            let func = &*func;
                            for i in 0..func.params.len() {
                                slots.insert(func.params[i].clone(), args[i].clone());
                            }
                        }

                        frames.push(CallFrame::new(func, slots));
                        should_update_counter = false;
                    } else if let Obj::NativeFunc(func) = func {
                        let arity = stack.pop().map(|obj| obj.as_const().as_num());

                        let mut args = vec![];

                        if let Some(arity) = arity {
                            for _ in 0..arity as u8 {
                                let arg = stack.pop().unwrap();
                                args.push(arg);
                            }
                        }

                        unsafe {
                            let func = &*func;
                            let res = (func.func)(arity.unwrap() as usize, args);
                            stack.push(res);
                        }
                    }
                }
                Bytecode::GetVar => {
                    let ident = stack.pop().unwrap().as_const().as_str();
                    let constant = scope_stack.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetVar => {
                    let ident = stack.pop().unwrap().as_const().as_str();
                    let lit = stack.pop().unwrap();
                    scope_stack.insert(&ident, lit);
                }
                Bytecode::DeclareClass => {
                    let fields_count = stack.pop().unwrap().as_const().as_num();
                    let class_name = stack.pop().unwrap().as_const().as_str();

                    let mut ptr = Box::new(ObjClass {
                        name: class_name.clone(),
                        fields_count: fields_count as u32,
                    });

                    let class = Obj::Class(ptr.as_mut() as *mut ObjClass);
                    scope_stack.insert(&class_name, class);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetProp => {
                    let prop = stack.pop().unwrap().as_const().as_str();
                    let this = stack.pop().unwrap();

                    if let Obj::Inst(inst) = this {
                        unsafe {
                            let inst = &*inst;
                            if let Some(method) = scope_stack.method(inst.class_ptr(), &prop) {
                                let mut b = Box::new(match method {
                                    Obj::Func(func) => ObjBoundMethod {
                                        this,
                                        func: ObjBoundMethodFunc::Default(func),
                                    },
                                    Obj::NativeFunc(native_func) => ObjBoundMethod {
                                        this,
                                        func: ObjBoundMethodFunc::Native(native_func),
                                    },
                                    _ => unreachable!(),
                                });

                                stack.push(Obj::BoundMethod(b.as_mut() as *mut ObjBoundMethod));

                                std::mem::forget(b);
                            } else {
                                let c = inst.prop(&prop).unwrap().clone();
                                stack.push(c);
                            }
                        }
                    } else if let Obj::Prim(prim) = this {
                        unsafe {
                            let inst = &*prim;
                            if let Some(method) = scope_stack.method(inst.class_ptr(), &prop) {
                                let mut b = Box::new(match method {
                                    Obj::Func(func) => ObjBoundMethod {
                                        this,
                                        func: ObjBoundMethodFunc::Default(func),
                                    },
                                    Obj::NativeFunc(native_func) => ObjBoundMethod {
                                        this,
                                        func: ObjBoundMethodFunc::Native(native_func),
                                    },
                                    _ => unreachable!(),
                                });

                                stack.push(Obj::BoundMethod(b.as_mut() as *mut ObjBoundMethod));

                                std::mem::forget(b);
                            }
                        }
                    }
                }
                Bytecode::SetProp => {
                    let inst = stack.pop().unwrap().as_instance();
                    let prop = stack.pop().unwrap().as_const().as_str();
                    let value = stack.pop().unwrap();

                    unsafe {
                        let inst = &mut *inst;
                        inst.set_prop(&prop, value);
                    }
                }
                Bytecode::InstantiateClass => {
                    let class_name = stack.pop().unwrap().as_const().as_str();
                    let class = scope_stack.get(&class_name).unwrap().as_class();
                    let mut props = HashMap::new();

                    unsafe {
                        let obj_class = &*class;

                        // REVIEW: I don't like this strategy, we're creating the fields based on their count.
                        for _ in 0..obj_class.fields_count {
                            let field_name = stack.pop().unwrap().as_const().as_str();
                            let field_value = stack.pop().unwrap();
                            props.insert(field_name, field_value);
                        }
                    }

                    let mut ptr = Box::new(ObjInst::new(class, props));
                    let instance = Obj::Inst(ptr.as_mut() as *mut ObjInst);

                    stack.push(instance);

                    std::mem::forget(ptr); // avoid dropping the value when going out of scope.
                }
                Bytecode::GetConst => {
                    let ident = stack.pop().unwrap().as_const().as_str();
                    let constant = scope_stack.get(&ident).unwrap();
                    stack.push(constant.clone());
                }
                Bytecode::SetConst => {
                    let ident = stack.pop().unwrap().as_const().as_str();
                    scope_stack.insert(&ident, stack.pop().unwrap());
                }
                Bytecode::Add => {
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::num(num_class_ptr, val1.value + val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Subtract => {
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::num(num_class_ptr, val1.value - val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Multiply => {
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::num(num_class_ptr, val1.value * val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Divide => {
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::num(num_class_ptr, val1.value / val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Equal => {
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::bool(num_class_ptr, val1.value == val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Greater => {
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::bool(num_class_ptr, val1.value > val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Less => {
                    let val1 = stack.pop().map(|obj| obj.as_prim()).unwrap();
                    let val2 = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val1;
                        let val2 = &*val2;

                        let mut res = ObjPrim::bool(num_class_ptr, val1.value < val2.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Negate => {
                    let val = stack.pop().map(|obj| obj.as_prim()).unwrap();

                    unsafe {
                        let val1 = &*val;

                        let mut res = ObjPrim::num(num_class_ptr, -val1.value);
                        let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        std::mem::forget(res);
                        stack.push(prim);
                    }
                }
                Bytecode::Not => {
                    let val = stack.pop().unwrap();

                    unsafe {
                        let prim = &*val.as_prim();

                        let method = scope_stack
                            .method(prim.class_ptr(), "toBool")
                            .unwrap()
                            .as_native_function();

                        let method = &*method;
                        let res = (method.func)(0, vec![]);
                        stack.push(res);

                        // match method.func {
                        //     ObjBoundMethodFunc::Default(func) => {}
                        //     ObjBoundMethodFunc::Native(native_func) => {
                        //         let native_func = &*native_func;
                        //         let res = (native_func.func)(0, vec![]);
                        //         stack.push(res);
                        //     }
                        // }

                        // let mut res = ObjPrim::bool(num_class_ptr, val.value == 0.0);
                        // let prim = Obj::Prim(&mut res as *mut ObjPrim);

                        // std::mem::forget(res);
                        // stack.push(prim);
                    }
                }
                Bytecode::Pop => {
                    stack.pop();
                }
                Bytecode::Return => {
                    frames.pop();
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
