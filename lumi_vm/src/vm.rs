use std::collections::HashMap;
use std::time::Instant;

use compiler::generator::bytecode::Bytecode;
use compiler::generator::chunk::Chunk;
use compiler::generator::constant::Constant;

use crate::call_frame::{CallFrame, CallFrameStack};
use crate::const_stack::ConstStack;
use crate::obj::{
    Obj, ObjBoundMethod, ObjBoundMethodFunc, ObjClass, ObjFunc, ObjInst, ObjNativeFunc, ObjPrim,
    ObjPrimKind, ObjStack,
};
use crate::raw_ptr::RawPtr;
use crate::scope::ScopeStack;

struct ObjFactory;

impl ObjFactory {
    fn create<T>(obj: T) -> *mut T {
        let mut boxed = Box::new(obj);
        let ptr = boxed.as_mut_ptr();

        // TODO: We should somehow register the objects in the garbage collector.
        std::mem::forget(boxed);
        ptr
    }
}

pub struct Vm {
    chunk: Chunk,
    frame_stack: CallFrameStack,
    scope_stack: ScopeStack,
    const_stack: ConstStack,
    obj_stack: ObjStack,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            frame_stack: CallFrameStack::new(),
            scope_stack: ScopeStack::new(),
            const_stack: ConstStack::new(),
            obj_stack: ObjStack::new(),
        }
    }

    pub fn run(&mut self) {
        let mut root_fun = Box::new(ObjFunc::root(self.chunk.clone()));
        let root_frame = CallFrame::new(root_fun.as_mut_ptr(), HashMap::new());

        self.frame_stack.push(root_frame);

        self.register_base_classes();
        self.register_base_functions();
        self.register_base_methods();

        let mut frame_has_changed = false;

        while let Some(inst) = self.frame_stack.current().and_then(|f| f.peek()) {
            match inst {
                Bytecode::LoadConstant(i) => {
                    let constant = self
                        .frame_stack
                        .current()
                        .and_then(|f| f.constant(i))
                        .unwrap();

                    self.const_stack.push(constant);
                }
                Bytecode::Lit => {
                    let constant = self.const_stack.pop();

                    let obj = match constant {
                        Constant::Nil => {
                            let nil_class = self.scope_stack.get("Nil").as_class();
                            ObjPrim::nil(nil_class)
                        }
                        Constant::Bool(b) => {
                            let bool_class = self.scope_stack.get("Bool").as_class();
                            ObjPrim::bool(bool_class, b)
                        }
                        Constant::Num(num) => {
                            let num_class = self.scope_stack.get("Num").as_class();
                            ObjPrim::num(num_class, num)
                        }
                        _ => todo!(),
                    };
                    self.obj_stack.push(Obj::Prim(ObjFactory::create(obj)))
                }
                Bytecode::InstantiateClass => {
                    let class_name = self.const_stack.pop().as_str();
                    let class_ptr = self.scope_stack.get(&class_name).as_class();

                    let mut props = HashMap::new();

                    unsafe {
                        let obj_class = &*class_ptr;

                        // REVIEW: I don't like this strategy, we're creating the fields based on their count.
                        for _ in 0..obj_class.fields_count {
                            let field_name = self.const_stack.pop().as_str();
                            let field_value = self.obj_stack.pop();
                            props.insert(field_name, field_value);
                        }
                    }

                    let obj_inst = ObjInst::new(class_ptr, props);
                    self.obj_stack.push(Obj::Inst(ObjFactory::create(obj_inst)));
                }
                Bytecode::BeginScope => {
                    self.scope_stack.push();

                    let current = self.frame_stack.current().unwrap();

                    for (key, object) in current.slots() {
                        self.scope_stack.insert(key, object.clone());
                    }
                }
                Bytecode::EndScope => {
                    self.scope_stack.pop();
                }
                Bytecode::DeclareFunc => {
                    let func_name = self.const_stack.pop().as_str();
                    let func_const = self.const_stack.pop().as_function();

                    // TODO: We can improve this initialization right?
                    let func = ObjFunc {
                        chunk: func_const.chunk().clone(),
                        name: func_const.name().clone(),
                        params: func_const.params().clone(),
                    };

                    let func_obj = Obj::Func(ObjFactory::create(func));
                    self.scope_stack.insert(&func_name, func_obj);
                }
                Bytecode::DeclareMethod => {
                    let class_name = self.const_stack.pop().as_str();
                    let func_name = self.const_stack.pop().as_str();
                    let func_const = self.const_stack.pop().as_function();

                    // TODO: We can improve this initialization right?
                    let func = ObjFunc {
                        chunk: func_const.chunk().clone(),
                        name: func_const.name().clone(),
                        params: func_const.params().clone(),
                    };

                    let class_ptr = self.scope_stack.get(&class_name).as_class();
                    let method_obj = Obj::Func(ObjFactory::create(func));
                    self.scope_stack
                        .set_method(class_ptr, &func_name, method_obj);
                }
                Bytecode::DeclareVar => {
                    let var_name = self.const_stack.pop().as_str();
                    let obj = self.obj_stack.pop();

                    self.scope_stack.insert(&var_name, obj);

                    let nil_class_ptr = self.scope_stack.get("Nil").as_class();

                    self.obj_stack
                        .push(Obj::Prim(ObjFactory::create(ObjPrim::nil(nil_class_ptr))));
                }
                Bytecode::DeclareConst => {
                    let var_name = self.const_stack.pop().as_str();
                    let obj = self.obj_stack.pop();
                    self.scope_stack.insert(&var_name, obj);
                }
                Bytecode::DeclareClass => {
                    let fields_count = self.const_stack.pop().as_num();
                    let class_name = self.const_stack.pop().as_str();

                    let class_ptr =
                        ObjFactory::create(ObjClass::new(&class_name, fields_count as u32));

                    self.scope_stack.insert(&class_name, Obj::Class(class_ptr));
                }
                Bytecode::SetProp => {
                    let instance = self.obj_stack.pop().as_instance();
                    let prop_name = self.const_stack.pop().as_str();
                    let obj = self.obj_stack.pop();

                    unsafe {
                        let instance = &mut *instance;
                        instance.set_prop(&prop_name, obj);
                    }

                    let nil_class_ptr = self.scope_stack.get("Nil").as_class();

                    self.obj_stack
                        .push(Obj::Prim(ObjFactory::create(ObjPrim::nil(nil_class_ptr))));
                }
                Bytecode::GetVar => {
                    let var_name = self.const_stack.pop().as_str();
                    let obj = self.scope_stack.get(&var_name);
                    self.obj_stack.push(obj);
                }
                Bytecode::GetConst => {
                    let var_name = self.const_stack.pop().as_str();
                    let obj = self.scope_stack.get(&var_name);
                    self.obj_stack.push(obj);
                }
                Bytecode::GetProp => {
                    let prop_name = self.const_stack.pop().as_str();
                    let obj = self.obj_stack.pop();

                    let class_ptr = match obj {
                        Obj::Inst(inst) => unsafe { (&*inst).class_ptr() },
                        Obj::Prim(prim) => unsafe { (&*prim).class_ptr() },
                        _ => unreachable!(),
                    };

                    if let Some(method) = self.scope_stack.method(class_ptr, &prop_name) {
                        let obj_bound_method = match method {
                            Obj::Func(func) => ObjBoundMethod {
                                this: obj,
                                func: ObjBoundMethodFunc::Default(func),
                            },
                            Obj::NativeFunc(native_func) => ObjBoundMethod {
                                this: obj,
                                func: ObjBoundMethodFunc::Native(native_func),
                            },
                            _ => unreachable!(),
                        };

                        let ptr = ObjFactory::create(obj_bound_method);
                        self.obj_stack.push(Obj::BoundMethod(ptr));
                    } else {
                        unsafe {
                            let instance = &*obj.as_instance();
                            let prop = instance.prop(&prop_name).unwrap();
                            self.obj_stack.push(prop);
                        }
                    }
                }
                Bytecode::Call => {
                    let obj = self.obj_stack.pop();

                    let params_count = self.const_stack.pop().as_num();

                    let mut args = vec![];
                    for _ in 0..params_count as u8 {
                        args.push(self.obj_stack.pop());
                    }

                    let mut slots = HashMap::new();

                    match obj {
                        Obj::Func(func) => {
                            unsafe {
                                let func = &*func;
                                for i in 0..func.params.len() {
                                    slots.insert(func.params[i].clone(), args[i].clone());
                                }
                            }

                            self.frame_stack.push(CallFrame::new(func, slots));
                            frame_has_changed = true;
                        }
                        Obj::NativeFunc(native_func) => unsafe {
                            let native_func = &*native_func;
                            self.obj_stack.push((native_func.func)(slots));
                        },
                        Obj::BoundMethod(bound_method) => unsafe {
                            let bound_method = &*bound_method;
                            let this = bound_method.this.clone();

                            slots.insert("this".to_owned(), this.clone());

                            let class_ptr = match this {
                                Obj::Inst(inst) => (&*inst).class_ptr(),
                                Obj::Prim(prim) => (&*prim).class_ptr(),
                                _ => unreachable!(),
                            };

                            slots.insert("This".to_owned(), Obj::Class(class_ptr));

                            match bound_method.func {
                                ObjBoundMethodFunc::Default(func_ptr) => {
                                    let func = &*func_ptr;
                                    for i in 0..func.params.len() {
                                        slots.insert(func.params[i].clone(), args[i].clone());
                                    }

                                    self.frame_stack.push(CallFrame::new(func_ptr, slots));
                                    frame_has_changed = true;
                                }
                                ObjBoundMethodFunc::Native(native_func_ptr) => {
                                    let native_func = &*native_func_ptr;
                                    self.obj_stack.push((native_func.func)(slots));
                                }
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                Bytecode::Return => {
                    // FIXME: All the functions need at least one return statement to work
                    self.frame_stack.pop();
                }
                Bytecode::Add => {
                    let operand2 = self.obj_stack.pop();
                    let operand1 = self.obj_stack.pop();

                    unsafe {
                        let class_ptr = match operand1 {
                            Obj::Inst(inst) => (&*inst).class_ptr(),
                            Obj::Prim(prim) => (&*prim).class_ptr(),
                            _ => unreachable!(),
                        };

                        let method = self.scope_stack.method(class_ptr, "add").unwrap();

                        let mut slots = HashMap::new();
                        slots.insert("this".to_owned(), operand1);
                        slots.insert("other".to_owned(), operand2);

                        match method {
                            Obj::Func(func) => {
                                self.frame_stack.push(CallFrame::new(func, slots));
                                frame_has_changed = true;
                            }
                            Obj::NativeFunc(native_func) => {
                                let native_func = &*native_func;
                                self.obj_stack.push((native_func.func)(slots));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Bytecode::Subtract => {
                    let operand2 = self.obj_stack.pop();
                    let operand1 = self.obj_stack.pop();

                    unsafe {
                        let class_ptr = match operand1 {
                            Obj::Inst(inst) => (&*inst).class_ptr(),
                            Obj::Prim(prim) => (&*prim).class_ptr(),
                            _ => unreachable!(),
                        };

                        let method = self.scope_stack.method(class_ptr, "sub").unwrap();

                        let mut slots = HashMap::new();
                        slots.insert("this".to_owned(), operand1);
                        slots.insert("other".to_owned(), operand2);

                        match method {
                            Obj::Func(func) => {
                                self.frame_stack.push(CallFrame::new(func, slots));
                                frame_has_changed = true;
                            }
                            Obj::NativeFunc(native_func) => {
                                let native_func = &*native_func;
                                self.obj_stack.push((native_func.func)(slots));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Bytecode::Println => {
                    let obj = self.obj_stack.pop();
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
                Bytecode::Pop => {
                    self.obj_stack.pop();
                }
                _ => {}
            }

            if frame_has_changed {
                frame_has_changed = false;
            } else {
                self.frame_stack.next();
            }
        }

        println!("{:#?}", self.scope_stack);
    }

    fn register_base_classes(&mut self) {
        let nil_class_ptr = ObjFactory::create(ObjClass::new("Nil", 0));
        let bool_class_ptr = ObjFactory::create(ObjClass::new("Bool", 0));
        let num_class_ptr = ObjFactory::create(ObjClass::new("Num", 0));

        self.scope_stack.insert("Nil", Obj::Class(nil_class_ptr));
        self.scope_stack.insert("Bool", Obj::Class(bool_class_ptr));
        self.scope_stack.insert("Num", Obj::Class(num_class_ptr));
    }

    fn register_base_functions(&mut self) {
        let start_time = Instant::now();
        let num_class_ptr = self.scope_stack.get("Num").as_class();

        let clock_native_func = ObjFactory::create(ObjNativeFunc::new("clock", move |_| -> Obj {
            let end_time = Instant::now();
            let elapsed_time = end_time - start_time;

            let res =
                ObjFactory::create(ObjPrim::num(num_class_ptr, elapsed_time.as_millis() as f64));
            Obj::Prim(res)
        }));

        self.scope_stack
            .insert("clock", Obj::NativeFunc(clock_native_func));
    }

    fn register_base_methods(&mut self) {
        let num_class_ptr = self.scope_stack.get("Num").as_class();

        let num_add_method = ObjFactory::create(ObjNativeFunc::new("add", move |params| -> Obj {
            let this = params.get("this").unwrap().as_prim();
            let other = params.get("other").unwrap().as_prim();

            unsafe {
                let this = &*this;
                let other = &*other;

                let res = ObjFactory::create(ObjPrim::num(num_class_ptr, this.value + other.value));
                Obj::Prim(res)
            }
        }));

        let num_sub_method = ObjFactory::create(ObjNativeFunc::new("sub", move |params| -> Obj {
            let this = params.get("this").unwrap().as_prim();
            let other = params.get("other").unwrap().as_prim();

            unsafe {
                let this = &*this;
                let other = &*other;

                let res = ObjFactory::create(ObjPrim::num(num_class_ptr, this.value - other.value));
                Obj::Prim(res)
            }
        }));

        let num_copy = ObjFactory::create(ObjNativeFunc::new("copy", move |params| -> Obj {
            let this = params.get("this").unwrap().as_prim();

            unsafe {
                let this = &*this;

                let res = ObjFactory::create(ObjPrim::num(num_class_ptr, this.value));
                Obj::Prim(res)
            }
        }));

        self.scope_stack
            .set_method(num_class_ptr, "add", Obj::NativeFunc(num_add_method));
        self.scope_stack
            .set_method(num_class_ptr, "sub", Obj::NativeFunc(num_sub_method));
        self.scope_stack
            .set_method(num_class_ptr, "copy", Obj::NativeFunc(num_copy));
    }
}
