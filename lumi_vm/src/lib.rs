mod call_frame;
mod const_stack;
mod obj_stack;
mod object;
mod runtime_error;
mod scope;

use std::{collections::HashMap, rc::Rc, time::Instant};

use call_frame::{CallFrame, CallFrameStack};
use const_stack::ConstStack;
use lumi_bc_e::chunk::{Bytecode, Chunk, Constant};
use obj_stack::ObjectStack;
use object::{Class, FromMut, FromPtr, Function, Instance, Object, Primitive};
use runtime_error::RuntimeError;
use scope::Scope;

use crate::object::Method;

struct GarbageCollector {}

impl GarbageCollector {
    fn register<T>(object: T) -> *mut T {
        let mut b = Box::new(object);
        let ptr = b.as_mut() as *mut T;
        std::mem::forget(b);
        ptr
    }
}

pub struct VirtualMachine {
    constant_stack: ConstStack,
    object_stack: ObjectStack,
    frame_stack: CallFrameStack,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            constant_stack: ConstStack::new(),
            object_stack: ObjectStack::new(),
            frame_stack: CallFrameStack::new(),
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), RuntimeError> {
        let nil_class_ptr = GarbageCollector::register(Class::new("Nil"));
        let num_class_ptr = GarbageCollector::register(Class::new("Num"));
        let bool_class_ptr = GarbageCollector::register(Class::new("Bool"));
        let start_time = Instant::now();
        let native_clock = Function::native("clock", &vec![], move |_| {
            let end_time = Instant::now();
            let elapsed_time = (end_time - start_time).as_millis() as f64;
            let n_ptr = GarbageCollector::register(Primitive::new(num_class_ptr, elapsed_time));
            Ok(Object::Primitive(n_ptr))
        });
        let mut cur_scope = Rc::new(Scope::root());
        cur_scope.set_symbol(
            &native_clock.name(),
            Object::Function(GarbageCollector::register(native_clock)),
        );
        self.frame_stack.push(CallFrame::new(
            cur_scope.clone(),
            0,
            chunk.len() - 1,
            HashMap::new(),
        ));
        while let (Some(frame), Some(bytecode)) = (
            self.frame_stack.current(),
            self.frame_stack
                .current()
                .and_then(|frame| chunk.bytecode(frame.start + frame.index)),
        ) {
            match bytecode {
                Bytecode::LoadConst => {
                    let constant = chunk.constant(frame.start + frame.index).unwrap();
                    self.constant_stack.push(constant.clone());
                    self.frame_stack.move_ptr(1 + Constant::index_size());
                }
                Bytecode::ConvertConst => {
                    let constant = self.constant_stack.pop()?;
                    match constant {
                        Constant::Nil => {
                            let nil_ptr =
                                GarbageCollector::register(Primitive::new(nil_class_ptr, 0.0));
                            let object = Object::Primitive(nil_ptr);
                            self.object_stack.push(object);
                        }
                        Constant::Bool(b) => {
                            let bool_ptr = GarbageCollector::register(Primitive::new(
                                bool_class_ptr,
                                if b { 1.0 } else { 0.0 },
                            ));
                            let object = Object::Primitive(bool_ptr);
                            self.object_stack.push(object);
                        }
                        Constant::Float(num) => {
                            let n_ptr =
                                GarbageCollector::register(Primitive::new(num_class_ptr, num));
                            let object = Object::Primitive(n_ptr);
                            self.object_stack.push(object);
                        }
                        _ => todo!("Strings"),
                    };
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::InstantiateClass => {
                    let fields_count = self.constant_stack.pop()?;
                    let mut fields = HashMap::new();
                    if let Constant::Size(fields_count) = fields_count {
                        let mut fields_count = fields_count as i16;
                        while fields_count > 0 {
                            let field_name = self.constant_stack.pop()?;
                            let field_value = self.object_stack.pop()?;
                            if let Constant::Str(field_name) = field_name {
                                fields.insert(field_name, field_value);
                            }
                            fields_count -= 1;
                        }
                    }
                    let class = self.object_stack.pop()?;
                    if let Object::Class(class) = class {
                        let instance = GarbageCollector::register(Instance::new(class, fields));
                        self.object_stack.push(Object::Instance(instance));
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::Println => {
                    let object = self.object_stack.pop()?;
                    match object {
                        Object::Primitive(prim) => {
                            println!("{:?}", prim.from_ptr().value());
                        }
                        Object::Instance(inst) => {
                            let i = inst.from_mut();
                            println!("<instance {}>", i.class().name());
                        }
                        _ => {}
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::DeclareVar => {
                    let object = self.object_stack.pop()?;
                    let var_name = self.constant_stack.pop()?;
                    if let Constant::Str(var_name) = var_name {
                        cur_scope.set_symbol(&var_name, object);
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::DeclareConst => {
                    let object = self.object_stack.pop()?;
                    let const_name = self.constant_stack.pop()?;
                    if let Constant::Str(const_name) = const_name {
                        cur_scope.set_symbol(&const_name, object);
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::DeclareClass => {
                    let class_name = self.constant_stack.pop()?;
                    if let Constant::Str(class_name) = class_name {
                        let tr = GarbageCollector::register(Class::new(&class_name));
                        cur_scope.set_symbol(&class_name, Object::Class(tr));
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::DeclareFun => {
                    let fun_name = self.constant_stack.pop()?;
                    let params_count = self.constant_stack.pop()?;
                    let mut params = vec![];
                    if let Constant::Size(params_count) = params_count {
                        for _ in 0..params_count {
                            let param_name = self.constant_stack.pop()?;
                            if let Constant::Str(param_name) = param_name {
                                params.push(param_name);
                            }
                        }
                    }
                    let start = self.constant_stack.pop()?;
                    let end = self.constant_stack.pop()?;
                    if let (Constant::Str(fun_name), Constant::Size(start), Constant::Size(end)) =
                        (fun_name, start, end)
                    {
                        let fun = GarbageCollector::register(Function::default(
                            cur_scope.clone(),
                            &fun_name,
                            start,
                            end,
                            &params,
                        ));
                        cur_scope.set_symbol(&fun_name, Object::Function(fun));
                        self.frame_stack.set_ptr(end);
                    }
                }
                Bytecode::DeclareMethod => {
                    let method_name = self.constant_stack.pop()?;
                    let params_count = self.constant_stack.pop()?;
                    let mut params = vec![];
                    if let Constant::Size(params_count) = params_count {
                        for _ in 0..params_count {
                            let param_name = self.constant_stack.pop()?;
                            if let Constant::Str(param_name) = param_name {
                                params.push(param_name);
                            }
                        }
                    }
                    let start = self.constant_stack.pop()?;
                    let end = self.constant_stack.pop()?;
                    let class_name = self.constant_stack.pop()?;
                    if let (
                        Constant::Str(class_name),
                        Constant::Str(method_name),
                        Constant::Size(start),
                        Constant::Size(end),
                    ) = (class_name, method_name, start, end)
                    {
                        let method = GarbageCollector::register(Function::default(
                            cur_scope.clone(),
                            &method_name,
                            start,
                            end,
                            &params,
                        ));
                        let class = cur_scope.symbol(&class_name)?;
                        if let Object::Class(class_ptr) = class {
                            cur_scope.set_method(class_ptr, &method_name, method);
                        }
                        self.frame_stack.set_ptr(end);
                    }
                }
                Bytecode::BeginScope => {
                    cur_scope = Rc::new(Scope::new(cur_scope));
                    for (key, object) in frame.slots() {
                        cur_scope.set_symbol(key, object.clone());
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::EndScope => {
                    cur_scope = cur_scope.parent.as_ref().unwrap().clone();
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::GetSymbol => {
                    let symbol_name = self.constant_stack.pop()?;
                    if let Constant::Str(symbol_name) = symbol_name {
                        let object = cur_scope.symbol(&symbol_name)?;
                        self.object_stack.push(object);
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::GetProp => {
                    let prop_name = self.constant_stack.pop()?;
                    let prop_value = self.object_stack.pop()?;
                    if let Some(class_ptr) = prop_value.class_ptr() {
                        if let (Constant::Str(prop_name), Object::Instance(instance)) =
                            (prop_name, prop_value.clone())
                        {
                            if let Ok(method) = cur_scope.method(class_ptr, &prop_name) {
                                let method_ptr =
                                    GarbageCollector::register(Method::new(prop_value, method));
                                self.object_stack.push(Object::Method(method_ptr));
                            } else {
                                let instance = instance.from_mut();
                                if let Some(prop) = instance.field(&prop_name) {
                                    self.object_stack.push(prop.clone());
                                }
                            }
                        }
                        self.frame_stack.move_ptr(1);
                    }
                }
                Bytecode::CallFun => {
                    let mut args = vec![];
                    let args_count = self.constant_stack.pop()?;
                    if let Constant::Size(args_count) = args_count {
                        let mut args_count = args_count as i16;
                        while args_count > 0 {
                            let arg = self.object_stack.pop()?;
                            args.push(arg);
                            args_count -= 1;
                        }
                    }
                    let callee = self.object_stack.pop()?;
                    if let Object::Function(function) = callee {
                        let function = function.from_ptr();
                        self.call(cur_scope.clone(), function, args, HashMap::new())?;
                        if let Function::Default { scope, .. } = function {
                            cur_scope = scope.clone()
                        }
                    } else if let Object::Method(method) = callee {
                        let method = method.from_ptr();
                        let mut symbols = HashMap::new();
                        symbols.insert("this".to_owned(), method.this().clone());
                        if let Some(class_ptr) = method.this().class_ptr() {
                            symbols.insert("This".to_owned(), Object::Class(class_ptr));
                        }
                        self.call(cur_scope.clone(), method.function(), args, symbols)?;
                        if let Function::Default { scope, .. } = method.function() {
                            cur_scope = scope.clone()
                        }
                    }
                }
                Bytecode::SetVar => {
                    let value = self.object_stack.pop()?;
                    let var_name = self.constant_stack.pop()?;
                    if let Constant::Str(var_name) = var_name {
                        cur_scope.assign_symbol(&var_name, value.clone())?;
                    }
                    self.object_stack.push(value);
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::SetProp => {
                    let object = self.object_stack.pop()?;
                    let value = self.object_stack.pop()?;
                    let prop_name = self.constant_stack.pop()?;
                    if let (Constant::Str(var_name), Object::Instance(instance)) =
                        (prop_name, object)
                    {
                        let instance = instance.from_mut();
                        instance.set_field(&var_name, value);
                    }
                    self.frame_stack.move_ptr(1);
                }
                Bytecode::JumpIfFalse => {
                    let offset = self.constant_stack.pop()?;
                    let object = self.object_stack.pop()?;
                    if let Object::Primitive(primitive) = object {
                        if primitive.from_ptr().value() == 0.0 {
                            if let Constant::Size(offset) = offset {
                                self.frame_stack.move_ptr(offset + 1);
                            }
                        } else {
                            self.frame_stack.move_ptr(1);
                        }
                    }
                }
                Bytecode::Add => {
                    let operand2 = self.object_stack.pop()?;
                    let operand1 = self.object_stack.pop()?;
                    if let (Object::Primitive(prim1), Object::Primitive(prim2)) =
                        (operand1, operand2)
                    {
                        let n_ptr = GarbageCollector::register(Primitive::new(
                            num_class_ptr,
                            prim1.from_ptr().value() + prim2.from_ptr().value(),
                        ));
                        let object = Object::Primitive(n_ptr);
                        self.object_stack.push(object);
                        self.frame_stack.move_ptr(1);
                    }
                }
                Bytecode::Subtract => {
                    let operand2 = self.object_stack.pop()?;
                    let operand1 = self.object_stack.pop()?;
                    if let (Object::Primitive(prim1), Object::Primitive(prim2)) =
                        (operand1, operand2)
                    {
                        let n_ptr = GarbageCollector::register(Primitive::new(
                            num_class_ptr,
                            prim1.from_ptr().value() - prim2.from_ptr().value(),
                        ));
                        let object = Object::Primitive(n_ptr);
                        self.object_stack.push(object);
                        self.frame_stack.move_ptr(1);
                    }
                }
                Bytecode::Equals => {
                    let operand2 = self.object_stack.pop()?;
                    let operand1 = self.object_stack.pop()?;
                    if let (Object::Primitive(operand1), Object::Primitive(operand2)) =
                        (operand1, operand2)
                    {
                        let bool_ptr = GarbageCollector::register(Primitive::new(
                            bool_class_ptr,
                            if operand1.from_ptr().value() == operand2.from_ptr().value() {
                                1.0
                            } else {
                                0.0
                            },
                        ));
                        let object = Object::Primitive(bool_ptr);
                        self.object_stack.push(object);
                        self.frame_stack.move_ptr(1);
                    }
                }
                Bytecode::Return => {
                    cur_scope = self.frame_stack.current().unwrap().root_scope.clone();
                    self.frame_stack.pop();
                    self.frame_stack.move_ptr(1);
                }
                other => {
                    println!("Bad bytecode {:?}", other);
                    self.frame_stack.move_ptr(1);
                }
            };
        }
        Ok(())
    }

    fn call(
        &mut self,
        scope: Rc<Scope>,
        function: &Function,
        args: Vec<Object>,
        symbols: HashMap<String, Object>,
    ) -> Result<(), RuntimeError> {
        let mut symbols = HashMap::from(symbols);
        for (index, param) in function.params().iter().enumerate() {
            symbols.insert(param.clone(), args[index].clone());
        }
        match function {
            Function::Default { start, end, .. } => {
                let frame = CallFrame::new(scope, *start, *end, symbols);
                self.frame_stack.push(frame);
            }
            Function::Native { fun, .. } => {
                let object = (fun)(symbols)?;
                self.object_stack.push(object);
                self.frame_stack.move_ptr(1);
            }
        }
        Ok(())
    }
}
