mod const_stack;
mod ctx;
mod frame;
mod obj;
mod obj_stack;
mod object;
mod runtime_error;
mod scope;

use std::{collections::HashMap, rc::Rc};

use ctx::ExecutionContext;
use frame::{CallFrame, Trace};
use lumi_bc_e::chunk::{Bytecode, Chunk, Constant};
use object::{Class, FromMut, FromPtr, Function, Instance, Object, Primitive};
use runtime_error::RuntimeError;
use scope::Scope;

use crate::object::Method;

struct GarbageCollector;

impl GarbageCollector {
    fn register<T>(object: T) -> *mut T {
        let mut b = Box::new(object);
        let ptr = b.as_mut() as *mut T;
        std::mem::forget(b);
        ptr
    }
}

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn run(chunk: Chunk) -> Result<(), RuntimeError> {
        let nil_class_ptr = GarbageCollector::register(Class::new("Nil"));
        let num_class_ptr = GarbageCollector::register(Class::new("Num"));
        let bool_class_ptr = GarbageCollector::register(Class::new("Bool"));
        let mut ctx = ExecutionContext::new(chunk);
        while let Some((frame, bytecode, bytecode_pos)) = ctx.next_bytecode() {
            match bytecode {
                Bytecode::LoadConst => {
                    let constant = ctx.chunk.constant(bytecode_pos).unwrap();
                    ctx.push_constant(constant.clone());
                    ctx.next_n_instruction(1 + Constant::index_size());
                }
                Bytecode::ConvertConst => {
                    let constant = ctx.pop_constant();
                    match constant {
                        Constant::Nil => {
                            let nil_ptr =
                                GarbageCollector::register(Primitive::new(nil_class_ptr, 0.0));
                            let object = Object::Primitive(nil_ptr);
                            ctx.push_object(object);
                        }
                        Constant::Bool(b) => {
                            let bool_ptr = GarbageCollector::register(Primitive::new(
                                bool_class_ptr,
                                if b { 1.0 } else { 0.0 },
                            ));
                            let object = Object::Primitive(bool_ptr);
                            ctx.push_object(object);
                        }
                        Constant::Float(num) => {
                            let n_ptr =
                                GarbageCollector::register(Primitive::new(num_class_ptr, num));
                            let object = Object::Primitive(n_ptr);
                            ctx.push_object(object);
                        }
                        _ => todo!("Strings"),
                    }
                    ctx.next_instruction();
                }
                Bytecode::InstantiateClass => {
                    let fields_count = ctx.pop_constant();
                    let mut fields = HashMap::new();
                    if let Constant::Size(fields_count) = fields_count {
                        let mut fields_count = fields_count as i16;
                        while fields_count > 0 {
                            let field_name = ctx.pop_constant();
                            let field_value = ctx.pop_object();
                            if let Constant::Str(field_name) = field_name {
                                fields.insert(field_name, field_value);
                            }
                            fields_count -= 1;
                        }
                    }
                    let class = ctx.pop_object();
                    if let Object::Class(class) = class {
                        let instance = GarbageCollector::register(Instance::new(class, fields));
                        ctx.push_object(Object::Instance(instance));
                        ctx.next_instruction();
                    } else {
                        let span = ctx.chunk.span(bytecode_pos).unwrap();
                        let stack_trace = ctx.call_stack.stack_trace();
                        return Err(RuntimeError::InvalidInstantiation {
                            symbol_name: "".to_string(),
                            span,
                            stack_trace,
                        });
                    }
                }
                Bytecode::Println => {
                    let object = ctx.pop_object();
                    match object {
                        Object::Primitive(prim) => {
                            println!("{:?}", prim.from_ptr().value());
                        }
                        Object::Instance(instance) => {
                            let inst = instance.from_mut();
                            println!("<instance {}>", inst.class().name());
                        }
                        _ => todo!("Print more data types"),
                    }
                    ctx.next_instruction();
                }
                Bytecode::DeclareVar => {
                    let object = ctx.pop_object();
                    let var_name = ctx.pop_constant();
                    if let Constant::Str(var_name) = var_name {
                        ctx.scope.set_symbol(&var_name, object);
                    }
                    ctx.next_instruction();
                }
                Bytecode::DeclareConst => {
                    let object = ctx.pop_object();
                    let const_name = ctx.pop_constant();
                    if let Constant::Str(const_name) = const_name {
                        ctx.scope.set_symbol(&const_name, object);
                    }
                    ctx.next_instruction();
                }
                Bytecode::DeclareClass => {
                    let class_name = ctx.pop_constant();
                    if let Constant::Str(class_name) = class_name {
                        let tr = GarbageCollector::register(Class::new(&class_name));
                        ctx.scope.set_symbol(&class_name, Object::Class(tr));
                    }
                    ctx.next_instruction();
                }
                Bytecode::DeclareFun => {
                    let fun_name = ctx.pop_constant();
                    let params_count = ctx.pop_constant();
                    let mut params = vec![];
                    if let Constant::Size(params_count) = params_count {
                        for _ in 0..params_count {
                            let param_name = ctx.pop_constant();
                            if let Constant::Str(param_name) = param_name {
                                params.push(param_name);
                            }
                        }
                    }
                    let start = ctx.pop_constant();
                    let end = ctx.pop_constant();
                    if let (Constant::Str(fun_name), Constant::Size(start), Constant::Size(end)) =
                        (fun_name, start, end)
                    {
                        let fun = GarbageCollector::register(Function::default(
                            Rc::clone(&ctx.scope),
                            &fun_name,
                            start,
                            end,
                            &params,
                        ));
                        ctx.scope.set_symbol(&fun_name, Object::Function(fun));
                        ctx.set_instruction(end);
                    }
                }
                Bytecode::DeclareMethod => {
                    let method_name = ctx.pop_constant();
                    let params_count = ctx.pop_constant();
                    let mut params = vec![];
                    if let Constant::Size(params_count) = params_count {
                        for _ in 0..params_count {
                            let param_name = ctx.pop_constant();
                            if let Constant::Str(param_name) = param_name {
                                params.push(param_name);
                            }
                        }
                    }
                    let start = ctx.pop_constant();
                    let end = ctx.pop_constant();
                    let class_name = ctx.pop_constant();
                    if let (
                        Constant::Str(class_name),
                        Constant::Str(method_name),
                        Constant::Size(start),
                        Constant::Size(end),
                    ) = (class_name, method_name, start, end)
                    {
                        let method = GarbageCollector::register(Function::default(
                            Rc::clone(&ctx.scope),
                            &method_name,
                            start,
                            end,
                            &params,
                        ));
                        if let Some(class) = ctx.scope.symbol(&class_name) {
                            if let Object::Class(class_ptr) = class {
                                ctx.scope.set_method(class_ptr, &method_name, method);
                            }
                            ctx.set_instruction(end);
                        } else {
                            let span = ctx.chunk.span(bytecode_pos).unwrap();
                            let stack_trace = ctx.call_stack.stack_trace();
                            return Err(RuntimeError::SymbolNotFound {
                                symbol_name: class_name,
                                span,
                                stack_trace,
                            });
                        }
                    }
                }
                Bytecode::BeginScope => {
                    let slots = frame.slots().clone();
                    ctx.add_scope(slots);
                    ctx.next_instruction();
                }
                Bytecode::EndScope => {
                    ctx.drop_scope();
                    ctx.next_instruction();
                }
                Bytecode::GetSymbol => {
                    let symbol_name = ctx.pop_constant();
                    if let Constant::Str(symbol_name) = symbol_name {
                        if let Some(object) = ctx.scope.symbol(&symbol_name) {
                            ctx.push_object(object);
                        } else {
                            let span = ctx.chunk.span(bytecode_pos).unwrap();
                            let stack_trace = ctx.call_stack.stack_trace();
                            return Err(RuntimeError::SymbolNotFound {
                                symbol_name: symbol_name,
                                span,
                                stack_trace,
                            });
                        }
                    }
                    ctx.next_instruction();
                }
                Bytecode::GetProp => {
                    let prop_name = ctx.pop_constant();
                    let prop_value = ctx.pop_object();
                    if let Some(class_ptr) = prop_value.class_ptr() {
                        if let (Constant::Str(prop_name), Object::Instance(instance)) =
                            (prop_name, prop_value.clone())
                        {
                            if let Some(method) = ctx.scope.method(class_ptr, &prop_name) {
                                let method_ptr =
                                    GarbageCollector::register(Method::new(prop_value, method));
                                ctx.push_object(Object::Method(method_ptr));
                            } else {
                                let instance = instance.from_mut();
                                if let Some(prop) = instance.field(&prop_name) {
                                    ctx.push_object(prop.clone());
                                } else {
                                    let span = ctx.chunk.span(bytecode_pos).unwrap();
                                    let stack_trace = ctx.call_stack.stack_trace();
                                    return Err(RuntimeError::CannotReadProperty {
                                        property_name: prop_name,
                                        class_name: class_ptr.from_ptr().name(),
                                        span,
                                        stack_trace,
                                    });
                                }
                            }
                        }
                        ctx.next_instruction();
                    }
                }
                Bytecode::SetVar => {
                    let object = ctx.pop_object();
                    let var_name = ctx.pop_constant();
                    if let Constant::Str(var_name) = var_name {
                        ctx.scope.assign_symbol(&var_name, object.clone());
                    }
                    ctx.push_object(object);
                    ctx.next_instruction();
                }
                Bytecode::SetProp => {
                    let object = ctx.pop_object();
                    let value = ctx.pop_object();
                    let prop_name = ctx.pop_constant();
                    if let (Constant::Str(var_name), Object::Instance(instance)) =
                        (prop_name, object)
                    {
                        let instance = instance.from_mut();
                        instance.set_field(&var_name, value);
                    }
                    ctx.next_instruction();
                }
                Bytecode::CallFun => {
                    let mut args = vec![];
                    let args_count = ctx.pop_constant();
                    if let Constant::Size(args_count) = args_count {
                        let mut args_count = args_count as i16;
                        while args_count > 0 {
                            let arg = ctx.pop_object();
                            args.push(arg);
                            args_count -= 1;
                        }
                    }
                    let callee = ctx.pop_object();
                    if let Object::Function(function) = callee {
                        let function = function.from_ptr();
                        VirtualMachine::call(
                            Rc::clone(&ctx.scope),
                            None,
                            function,
                            args,
                            HashMap::new(),
                            &mut ctx,
                            bytecode_pos,
                        )?;
                        if let Function::Default { scope, .. } = function {
                            ctx.scope = scope.clone()
                        }
                    } else if let Object::Method(method) = callee {
                        let method = method.from_ptr();
                        let mut symbols = HashMap::new();
                        symbols.insert("this".to_owned(), method.this().clone());
                        if let Some(class_ptr) = method.this().class_ptr() {
                            symbols.insert("This".to_owned(), Object::Class(class_ptr));
                        }
                        VirtualMachine::call(
                            Rc::clone(&ctx.scope),
                            Some(method.this().class_ptr().unwrap().from_ptr()),
                            method.function(),
                            args,
                            symbols,
                            &mut ctx,
                            bytecode_pos,
                        )?;
                        if let Function::Default { scope, .. } = method.function() {
                            ctx.scope = scope.clone()
                        }
                    } else {
                        let span = ctx.chunk.span(bytecode_pos).unwrap();
                        let stack_trace = ctx.call_stack.stack_trace();
                        return Err(RuntimeError::SymbolNotCallable {
                            symbol_name: "".to_string(),
                            span,
                            stack_trace,
                        });
                    }
                }
                Bytecode::JumpIfFalse => {
                    let offset = ctx.pop_constant();
                    let object = ctx.pop_object();
                    if let Object::Primitive(primitive) = object {
                        if primitive.from_ptr().value() == 0.0 {
                            if let Constant::Size(offset) = offset {
                                ctx.next_n_instruction(offset + 1)
                            }
                        } else {
                            ctx.next_instruction();
                        }
                    } else {
                        ctx.next_instruction();
                    }
                }
                Bytecode::Add => {
                    let operand2 = ctx.pop_object();
                    let operand1 = ctx.pop_object();
                    if let (Object::Primitive(prim1), Object::Primitive(prim2)) =
                        (operand1.clone(), operand2.clone())
                    {
                        let n_ptr = GarbageCollector::register(Primitive::new(
                            num_class_ptr,
                            prim1.from_ptr().value() + prim2.from_ptr().value(),
                        ));
                        let object = Object::Primitive(n_ptr);
                        ctx.push_object(object);
                        ctx.next_instruction();
                    } else {
                        let span = ctx.chunk.span(bytecode_pos).unwrap();
                        let stack_trace = ctx.call_stack.stack_trace();
                        return Err(RuntimeError::InvalidBinaryOperands {
                            type_name_left: "".to_string(),
                            type_name_right: "".to_string(),
                            span,
                            stack_trace,
                        });
                    }
                }
                Bytecode::Subtract => {
                    let operand2 = ctx.pop_object();
                    let operand1 = ctx.pop_object();
                    if let (Object::Primitive(prim1), Object::Primitive(prim2)) =
                        (operand1.clone(), operand2.clone())
                    {
                        let n_ptr = GarbageCollector::register(Primitive::new(
                            num_class_ptr,
                            prim1.from_ptr().value() - prim2.from_ptr().value(),
                        ));
                        let object = Object::Primitive(n_ptr);
                        ctx.push_object(object);
                        ctx.next_instruction();
                    } else {
                        let span = ctx.chunk.span(bytecode_pos).unwrap();
                        let stack_trace = ctx.call_stack.stack_trace();
                        return Err(RuntimeError::InvalidBinaryOperands {
                            type_name_left: "".to_string(),
                            type_name_right: "".to_string(),
                            span,
                            stack_trace,
                        });
                    }
                }
                Bytecode::Equals => {
                    let operand2 = ctx.pop_object();
                    let operand1 = ctx.pop_object();
                    if let (Object::Primitive(operand1), Object::Primitive(operand2)) =
                        (operand1.clone(), operand2.clone())
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
                        ctx.push_object(object);
                        ctx.next_instruction();
                    } else {
                        let span = ctx.chunk.span(bytecode_pos).unwrap();
                        let stack_trace = ctx.call_stack.stack_trace();
                        return Err(RuntimeError::InvalidBinaryOperands {
                            type_name_left: "".to_string(),
                            type_name_right: "".to_string(),
                            span,
                            stack_trace,
                        });
                    }
                }
                Bytecode::Return => {
                    ctx.pop_frame();
                    ctx.next_instruction();
                }
                other => {
                    println!("Bad bytecode {:?}", other);
                    ctx.next_instruction();
                }
            }
        }
        Ok(())
    }

    fn call(
        scope: Rc<Scope>,
        class: Option<&Class>,
        function: &Function,
        args: Vec<Object>,
        symbols: HashMap<String, Object>,
        ctx: &mut ExecutionContext,
        bytecode_pos: usize,
    ) -> Result<(), RuntimeError> {
        let mut symbols = HashMap::from(symbols);
        for (index, arg) in args.iter().enumerate() {
            symbols.insert(function.params()[index].clone(), arg.clone());
        }
        match function {
            Function::Default { start, end, .. } => {
                let span = ctx.chunk.span(bytecode_pos).unwrap();
                let frame = if let Some(class) = class {
                    CallFrame::new(
                        scope,
                        *start,
                        *end,
                        symbols,
                        Some(Trace::for_method(
                            &class.name(),
                            &function.name(),
                            span.clone(),
                        )),
                    )
                } else {
                    CallFrame::new(
                        scope,
                        *start,
                        *end,
                        symbols,
                        Some(Trace::for_function(&function.name(), span.clone())),
                    )
                };
                ctx.push_frame(frame);
            }
            Function::Native { fun, .. } => {
                let object = (fun)(symbols)?;
                ctx.push_object(object);
                ctx.next_instruction();
            }
        }
        Ok(())
    }
}
