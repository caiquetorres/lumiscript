use std::{collections::HashMap, rc::Rc, time::Instant};

use crate::{
    call_frame::{CallFrame, CallStack},
    chunk::{Bytecode, Chunk, Constant},
    memory::Memory,
    object::{Class, Function, InnerFunction, Instance, NativeFunction, Object, Primitive},
    runtime_error::RuntimeError,
    scope::Scope,
    stack_trace::{StackTrace, Trace, TraceFunction},
};

pub struct Vm {
    chunk: Chunk,
    memory: Memory,
    call_stack: CallStack,
    object_stack: Vec<usize>,
    constant_stack: Vec<Constant>,
    stack_trace: StackTrace,
    scope: Rc<Scope>,
}

impl Vm {
    fn get_object(&self, object_id: usize) -> &Object {
        self.memory.get(object_id)
    }

    fn scope(&self) -> &Rc<Scope> {
        &self.scope
    }

    fn push_constant(&mut self, constant: Constant) {
        self.constant_stack.push(constant)
    }

    fn push_object(&mut self, object_id: usize) {
        self.object_stack.push(object_id)
    }

    fn create_object(&mut self, object: Object) -> (usize, &Object) {
        let object_id = self.memory.alloc(object);
        self.object_stack.push(object_id);
        let object = self.memory.get(object_id);
        (object_id, object)
    }

    fn pop_constant(&mut self) -> Constant {
        self.constant_stack.pop().unwrap()
    }

    fn pop_object(&mut self) -> (usize, &Object) {
        let object_id = self.object_stack.pop().unwrap();
        let object = self.memory.get(object_id);
        (object_id, object)
    }
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        let root_call_frame = CallFrame::new(None, HashMap::new());
        Self {
            chunk,
            memory: Memory::new(),
            call_stack: CallStack::new(root_call_frame),
            object_stack: vec![],
            constant_stack: vec![],
            stack_trace: StackTrace::new(),
            scope: Rc::new(Scope::root()),
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        register(self);
        while let Some(instruction) = self.current_instruction() {
            match instruction {
                Bytecode::LoadConstant => op_load_constant(self)?,
                Bytecode::ConvertConstant => op_convert_constant(self)?,
                Bytecode::DeclareClass => op_declare_class(self)?,
                Bytecode::Instantiate => op_instantiate(self)?,
                Bytecode::PrintLn => op_println(self)?,
                Bytecode::GetSymbol => op_get_symbol(self)?,
                Bytecode::DeclareVariable => op_declare_var(self)?,
                Bytecode::SetVariable => op_set_var(self)?,
                Bytecode::Return => op_return(self)?,
                Bytecode::BeginScope => op_begin_scope(self)?,
                Bytecode::EndScope => op_end_scope(self)?,
                Bytecode::SetProperty => op_set_property(self)?,
                Bytecode::GetProperty => op_get_property(self)?,
                Bytecode::DeclareFunction => op_declare_function(self)?,
                Bytecode::CallFunction => op_call_function(self)?,
                Bytecode::DeclareMethod => op_declare_method(self)?,
                Bytecode::Add => op_add(self)?,
                Bytecode::Subtract => op_sub(self)?,
                Bytecode::Equals => op_eq(self)?,
                Bytecode::Not => op_not(self)?,
                Bytecode::JumpIfFalse => op_jump_if_false(self)?,
                Bytecode::Pop => op_pop(self)?,
                _ => panic!("Bytecode {:?} not implemented", instruction),
            };
        }
        if !self.object_stack.is_empty() {
            println!("Stack not empty, {:?}", self.object_stack);
        }
        Ok(())
    }

    fn register_native_function(
        &mut self,
        function_name: &str,
        params: &[String],
        function: NativeFunction,
    ) {
        let function_id = self.memory.alloc(Object::Function(Function::new(
            function_name,
            params,
            None,
            InnerFunction::Native { fun: function },
        )));
        self.scope.set_symbol(function_name, function_id);
    }

    fn register_native_method(
        &mut self,
        class_id: usize,
        method_name: &str,
        params: &[String],
        function: NativeFunction,
    ) {
        let method_id = self.memory.alloc(Object::Function(Function::new(
            method_name,
            params,
            Some(class_id),
            InnerFunction::Native { fun: function },
        )));
        self.scope.set_method(class_id, method_name, method_id);
    }

    fn frame(&self) -> &CallFrame {
        self.call_stack.current()
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.call_stack.current_mut()
    }

    fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    fn current_instruction(&self) -> Option<Bytecode> {
        self.chunk().instruction(self.frame().instructions_ptr)
    }
}

fn register(vm: &mut Vm) {
    let nil_ptr = vm.memory.alloc(Object::Class(Class::new("Nil")));
    let bool_ptr = vm.memory.alloc(Object::Class(Class::new("Bool")));
    let num_ptr = vm.memory.alloc(Object::Class(Class::new("Number")));
    vm.scope.set_symbol("Nil", nil_ptr);
    vm.scope.set_symbol("Bool", bool_ptr);
    vm.scope.set_symbol("Num", num_ptr);
    let start = Instant::now();
    vm.register_native_function(
        "clock",
        &vec![],
        Box::new(move |_, _| {
            let end = Instant::now();
            let diff = (end - start).as_millis();
            Ok(Object::Primitive(Primitive::new(2, diff as f64)))
        }),
    );

    vm.register_native_method(
        2,
        "add",
        &vec!["other".to_owned()],
        Box::new(|vm, params| {
            let this = *params.get("this").unwrap();
            let other = *params.get("other").unwrap();
            let this = vm.memory.get(this);
            let other = vm.memory.get(other);
            if let (Object::Primitive(operand1), Object::Primitive(operand2)) = (this, other) {
                Ok(Object::Primitive(Primitive::new(
                    2,
                    operand1.value() + operand2.value(),
                )))
            } else {
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                Err(RuntimeError::InvalidBinaryOperands {
                    span: span.clone(),
                    stack_trace: vm.stack_trace.clone(),
                })
            }
        }),
    );

    vm.register_native_method(
        2,
        "sub",
        &vec!["other".to_owned()],
        Box::new(|vm, params| {
            let this = *params.get("this").unwrap();
            let other = *params.get("other").unwrap();
            let this = vm.memory.get(this);
            let other = vm.memory.get(other);
            if let (Object::Primitive(operand1), Object::Primitive(operand2)) = (this, other) {
                Ok(Object::Primitive(Primitive::new(
                    2,
                    operand1.value() - operand2.value(),
                )))
            } else {
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                Err(RuntimeError::InvalidBinaryOperands {
                    span: span.clone(),
                    stack_trace: vm.stack_trace.clone(),
                })
            }
        }),
    );

    vm.register_native_method(
        2,
        "eq",
        &["other".to_owned()],
        Box::new(|vm, params| {
            let this = *params.get("this").unwrap();
            let other = *params.get("other").unwrap();
            let this = vm.memory.get(this);
            let other = vm.memory.get(other);
            if let (Object::Primitive(operand1), Object::Primitive(operand2)) = (this, other) {
                Ok(Object::Primitive(Primitive::new(
                    1,
                    if operand1.value() == operand2.value() {
                        1.0
                    } else {
                        0.0
                    },
                )))
            } else {
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                Err(RuntimeError::InvalidBinaryOperands {
                    span: span.clone(),
                    stack_trace: vm.stack_trace.clone(),
                })
            }
        }),
    );

    vm.register_native_method(
        1,
        "not",
        &[],
        Box::new(|vm, params| {
            let this = *params.get("this").unwrap();
            let this = vm.memory.get(this);
            if let Object::Primitive(primitive) = this {
                Ok(Object::Primitive(Primitive::new(
                    1,
                    if primitive.value() == 0.0 { 1.0 } else { 0.0 },
                )))
            } else {
                unreachable!()
            }
        }),
    );
}

fn op_begin_scope(vm: &mut Vm) -> Result<(), RuntimeError> {
    let slots = vm.frame().slots();
    // REVIEW: Maybe we should created a method/function for that.
    vm.scope = Rc::new(Scope::new(Rc::clone(&vm.scope)));
    for (key, object) in &slots {
        vm.scope.set_symbol(key, object.clone());
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_end_scope(vm: &mut Vm) -> Result<(), RuntimeError> {
    if let Some(parent) = &vm.scope.parent {
        vm.scope = Rc::clone(&parent);
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_load_constant(vm: &mut Vm) -> Result<(), RuntimeError> {
    let constant = vm.chunk().constant(vm.frame().instructions_ptr).unwrap();
    vm.push_constant(constant.clone());
    vm.frame_mut().instructions_ptr += 4; // 1 + constant size
    Ok(())
}

fn op_convert_constant(vm: &mut Vm) -> Result<(), RuntimeError> {
    let constant = vm.constant_stack.pop().unwrap();
    match constant {
        Constant::Nil => {
            vm.create_object(Object::Primitive(Primitive::new(0, 0.0)));
        }
        Constant::Bool(value) => {
            vm.create_object(Object::Primitive(Primitive::new(
                1,
                if value { 1.0 } else { 0.0 },
            )));
        }
        Constant::Number(value) => {
            vm.create_object(Object::Primitive(Primitive::new(2, value)));
        }
        _ => panic!("Cannot convert to a value"),
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_declare_var(vm: &mut Vm) -> Result<(), RuntimeError> {
    let (object_id, _) = vm.pop_object();
    let variable_name = vm.pop_constant().as_string();
    vm.scope().set_symbol(&variable_name, object_id);
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_declare_class(vm: &mut Vm) -> Result<(), RuntimeError> {
    let class_name = vm.pop_constant().as_string();
    let class = Object::Class(Class::new(&class_name));
    let class_id = vm.memory.alloc(class);
    vm.scope.set_symbol(&class_name, class_id);
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_instantiate(vm: &mut Vm) -> Result<(), RuntimeError> {
    let fields_count = vm.pop_constant().as_size();
    let mut fields = HashMap::new();
    let mut fields_count = fields_count as i16;
    while fields_count > 0 {
        let field_name = vm.pop_constant().as_string();
        let (field_value_id, _) = vm.pop_object();
        fields.insert(field_name, field_value_id);
        fields_count -= 1;
    }
    let (class_id, class) = vm.pop_object();
    if class_id == 0 || class_id == 1 || class_id == 2 {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::Custom {
            message: "cannot instantiate a primitive type".to_owned(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    } else if let Object::Class(_) = class {
        vm.create_object(Object::Instance(Instance::new(class_id, fields)));
        vm.frame_mut().instructions_ptr += 1;
        Ok(())
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::InvalidInstantiation {
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_println(vm: &mut Vm) -> Result<(), RuntimeError> {
    let (object_id, _) = vm.pop_object();
    match vm.get_object(object_id) {
        Object::Class(class) => {
            println!("<class {}>", class.name());
        }
        Object::Primitive(primitive) => {
            println!("{:?}", primitive.value());
        }
        Object::Function(_) => {
            println!("<function>");
        }
        Object::Instance(instance) => {
            let class = vm.get_object(instance.class_id());
            if let Object::Class(class) = class {
                println!("<instance {}>", class.name());
            }
        }
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_get_symbol(vm: &mut Vm) -> Result<(), RuntimeError> {
    let symbol_name = vm.pop_constant().as_string();
    if let Some(object) = vm.scope.symbol(&symbol_name) {
        vm.push_object(object);
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        return Err(RuntimeError::SymbolNotFound {
            symbol_name,
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        });
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_set_var(vm: &mut Vm) -> Result<(), RuntimeError> {
    let (object_id, _) = vm.pop_object();
    let var_name = vm.pop_constant().as_string();
    vm.scope.assign_symbol(&var_name, object_id);
    vm.push_object(object_id);
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_return(vm: &mut Vm) -> Result<(), RuntimeError> {
    if let Some(return_scope) = vm.frame().return_scope() {
        vm.scope = Rc::clone(&return_scope);
    }
    vm.call_stack.pop();
    vm.stack_trace.pop();
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_set_property(vm: &mut Vm) -> Result<(), RuntimeError> {
    let (lhs_id, _) = vm.pop_object();
    let (rhs_id, _) = vm.pop_object();
    let prop_name = vm.pop_constant().as_string();
    if let Object::Instance(instance) = vm.memory.get_mut(lhs_id) {
        instance.set_field(&prop_name, rhs_id);
    }
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}

fn op_get_property(vm: &mut Vm) -> Result<(), RuntimeError> {
    let prop_name = vm.pop_constant().as_string();
    let instance_id = vm.object_stack.pop().unwrap();
    let instance = vm.memory.get(instance_id);
    match instance {
        Object::Instance(instance) => {
            if let Some(prop) = instance.field(&prop_name) {
                vm.object_stack.push(prop);
            } else if let Some(method) = vm.scope.method(instance.class_id(), &prop_name) {
                vm.object_stack.push(instance_id);
                vm.object_stack.push(method);
            } else {
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                let class = vm.memory.get(instance.class_id());
                if let Object::Class(class) = class {
                    return Err(RuntimeError::CannotReadProperty {
                        property_name: span.source_text(),
                        class_name: class.name(),
                        span: span.clone(),
                        stack_trace: vm.stack_trace.clone(),
                    });
                }
            }
            vm.frame_mut().instructions_ptr += 1;
            Ok(())
        }
        Object::Primitive(instance) => {
            if let Some(method) = vm.scope.method(instance.class(), &prop_name) {
                vm.object_stack.push(instance_id);
                vm.object_stack.push(method);
            } else {
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                let class = vm.memory.get(instance.class());
                if let Object::Class(class) = class {
                    return Err(RuntimeError::CannotReadProperty {
                        property_name: span.source_text(),
                        class_name: class.name(),
                        span: span.clone(),
                        stack_trace: vm.stack_trace.clone(),
                    });
                }
            }
            vm.frame_mut().instructions_ptr += 1;
            Ok(())
        }
        Object::Class(class) => {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::CannotReadProperty {
                property_name: span.source_text(),
                class_name: class.name(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
        Object::Function(_) => {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::CannotReadProperty {
                property_name: span.source_text(),
                class_name: "Function".to_owned(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
    }
}

fn op_declare_function(vm: &mut Vm) -> Result<(), RuntimeError> {
    let function_name = vm.constant_stack.pop().unwrap();
    let params_count = vm.constant_stack.pop().unwrap();
    let mut params = vec![];
    if let Constant::Size(params_count) = params_count {
        for _ in 0..params_count {
            let param_name = vm.constant_stack.pop().unwrap();
            if let Constant::String(param_name) = param_name {
                params.push(param_name);
            }
        }
    }
    let start = vm.constant_stack.pop().unwrap();
    let end = vm.constant_stack.pop().unwrap();
    if let (Constant::String(function_name), Constant::Size(start), Constant::Size(end)) =
        (function_name, start, end)
    {
        let object = Object::Function(Function::new(
            &function_name,
            &params,
            None,
            InnerFunction::frame(Rc::clone(&vm.scope), start..end),
        ));
        let object_id = vm.memory.alloc(object);
        vm.object_stack.push(object_id);
        vm.scope.set_symbol(&function_name, object_id);
        vm.frame_mut().instructions_ptr = end;
    }
    Ok(())
}

fn op_declare_method(vm: &mut Vm) -> Result<(), RuntimeError> {
    let method_name = vm.constant_stack.pop().unwrap();
    let params_count = vm.constant_stack.pop().unwrap();
    let mut params = vec![];
    if let Constant::Size(params_count) = params_count {
        for _ in 0..params_count {
            let param_name = vm.constant_stack.pop().unwrap();
            if let Constant::String(param_name) = param_name {
                params.push(param_name);
            }
        }
    }
    let start = vm.constant_stack.pop().unwrap();
    let end = vm.constant_stack.pop().unwrap();
    let class_name = vm.constant_stack.pop().unwrap();
    if let (
        Constant::String(class_name),
        Constant::String(method_name),
        Constant::Size(start),
        Constant::Size(end),
    ) = (class_name, method_name, start, end)
    {
        if let Some(class) = vm.scope.symbol(&class_name) {
            let method_id = vm.memory.alloc(Object::Function(Function::new(
                &method_name,
                &params,
                Some(class),
                InnerFunction::Frame {
                    scope: Rc::clone(&vm.scope),
                    range: start..end,
                },
            )));
            vm.scope.set_method(class, &method_name, method_id);
            vm.frame_mut().instructions_ptr = end;
        } else {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            return Err(RuntimeError::SymbolNotFound {
                symbol_name: span.source_text(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            });
        }
    } else {
        panic!("Cannot convert constants");
    }
    Ok(())
}

fn op_call_function(vm: &mut Vm) -> Result<(), RuntimeError> {
    let mut args = vec![];
    let args_count = vm.constant_stack.pop().unwrap();
    if let Constant::Size(args_count) = args_count {
        let mut args_count = args_count as i16;
        while args_count > 0 {
            let arg = vm.object_stack.pop().unwrap();
            args.push(arg);
            args_count -= 1;
        }
    }
    let callee_id = vm.object_stack.pop().unwrap();
    call_function(vm, &args, callee_id)
}

fn op_add(vm: &mut Vm) -> Result<(), RuntimeError> {
    let operand2 = vm.object_stack.pop().unwrap();
    let operand1 = *vm.object_stack.last().unwrap();
    let object1 = vm.memory.get(operand1);
    if let Some(class_id) = object1.class_id() {
        if let Some(method) = vm.scope.method(class_id, "add") {
            call_function(vm, &[operand2], method)
        } else {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::Custom {
                message: "trait \"Add\" not implemented".to_owned(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::Custom {
            message: "trait \"Add\" not implemented".to_owned(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_sub(vm: &mut Vm) -> Result<(), RuntimeError> {
    let operand2 = vm.object_stack.pop().unwrap();
    let operand1 = *vm.object_stack.last().unwrap();
    let object1 = vm.memory.get(operand1);
    if let Some(class_id) = object1.class_id() {
        if let Some(method) = vm.scope.method(class_id, "sub") {
            call_function(vm, &[operand2], method)
        } else {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::Custom {
                message: "trait \"Sub\" not implemented".to_owned(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::Custom {
            message: "trait \"Sub\" not implemented".to_owned(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_eq(vm: &mut Vm) -> Result<(), RuntimeError> {
    let operand2 = vm.object_stack.pop().unwrap();
    let operand1 = *vm.object_stack.last().unwrap();
    let object1 = vm.memory.get(operand1);
    if let Some(class_id) = object1.class_id() {
        if let Some(method) = vm.scope.method(class_id, "eq") {
            call_function(vm, &[operand2], method)
        } else {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::Custom {
                message: "trait \"Equals\" not implemented".to_owned(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::Custom {
            message: "trait \"Equals\" not implemented".to_owned(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_not(vm: &mut Vm) -> Result<(), RuntimeError> {
    let object_id = *vm.object_stack.last().unwrap();
    let object = vm.memory.get(object_id);
    if let Some(class_id) = object.class_id() {
        if let Some(method) = vm.scope.method(class_id, "not") {
            call_function(vm, &[], method)
        } else {
            let index = vm.frame().instructions_ptr;
            let span = vm.chunk().span(index);
            Err(RuntimeError::Custom {
                message: "trait \"Not\" not implemented".to_owned(),
                span: span.clone(),
                stack_trace: vm.stack_trace.clone(),
            })
        }
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::Custom {
            message: "trait \"Not\" not implemented".to_owned(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_jump_if_false(vm: &mut Vm) -> Result<(), RuntimeError> {
    let offset = vm.constant_stack.pop().unwrap();
    let object_id = vm.object_stack.pop().unwrap();
    if let Object::Primitive(primitive) = vm.memory.get(object_id) {
        if primitive.value() == 0.0 {
            if let Constant::Size(offset) = offset {
                vm.frame_mut().instructions_ptr += offset + 1;
            }
        } else {
            vm.frame_mut().instructions_ptr += 1;
        }
    }
    Ok(())
}

fn call_function(vm: &mut Vm, args: &[usize], callee_id: usize) -> Result<(), RuntimeError> {
    if let Object::Function(function) = vm.memory.get(callee_id) {
        let mut symbols = HashMap::new();
        for (index, arg) in args[..function.params().len()].iter().enumerate() {
            symbols.insert(function.params()[index].clone(), arg.clone());
        }
        if let Some(class_id) = function.class() {
            let instance = vm.object_stack.pop().unwrap();
            symbols.insert("this".to_owned(), instance);
            symbols.insert("This".to_owned(), class_id);
        }
        let class = function.class().map(|class_id| vm.memory.get(class_id));
        match function.inner() {
            InnerFunction::Frame { range, scope } => {
                let frame = CallFrame::new(Some(Rc::clone(&vm.scope)), symbols);
                let index = vm.frame().instructions_ptr;
                let span = vm.chunk().span(index);
                if let Some(class) = class {
                    if let Object::Class(class) = class {
                        vm.stack_trace.push(Trace::new(
                            span.clone(),
                            Some(TraceFunction::new(&function.name(), Some(&class.name()))),
                        ));
                    }
                } else {
                    vm.stack_trace.push(Trace::new(
                        span.clone(),
                        Some(TraceFunction::new(&function.name(), None)),
                    ));
                }
                vm.call_stack.push(frame);
                vm.scope = Rc::clone(&scope);
                vm.frame_mut().instructions_ptr = range.start;
            }
            InnerFunction::Native { fun } => {
                let object = (fun)(vm, symbols)?;
                let object_id = vm.memory.alloc(object);
                vm.object_stack.push(object_id);
                vm.frame_mut().instructions_ptr += 1;
            }
        }
        Ok(())
    } else {
        let index = vm.frame().instructions_ptr;
        let span = vm.chunk().span(index);
        Err(RuntimeError::SymbolNotCallable {
            symbol_name: span.source_text(),
            span: span.clone(),
            stack_trace: vm.stack_trace.clone(),
        })
    }
}

fn op_pop(vm: &mut Vm) -> Result<(), RuntimeError> {
    vm.pop_object();
    vm.frame_mut().instructions_ptr += 1;
    Ok(())
}
