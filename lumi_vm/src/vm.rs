use std::collections::HashMap;
use std::time::Instant;

use compiler::generator::bytecode::Bytecode;
use compiler::generator::chunk::Chunk;

use crate::call_frame::{CallFrame, CallFrameStack};
use crate::obj::{
    Obj, ObjBoundMethod, ObjBoundMethodFunc, ObjClass, ObjFunc, ObjInst, ObjNativeFunc, ObjPrim,
    ObjPrimKind,
};

use crate::operations::add::Add;
use crate::operations::begin_scope::BeginScope;
use crate::operations::declare_func::DeclareFunc;
use crate::operations::declare_method::DeclareMethod;
use crate::operations::declare_trait::DeclareTrait;
use crate::operations::end_scope::EndScope;
use crate::operations::impl_trait::ImplTrait;
use crate::operations::lit::Lit;
use crate::operations::load_constant::LoadConstant;
use crate::operations::pop::Pop;
use crate::operations::substract::Subtract;
use crate::raw_ptr::RawPtr;
use crate::runtime_error::RuntimeError;
use crate::scope::ScopeStack;
use crate::stacks::const_stack::ConstStack;
use crate::stacks::obj_stack::ObjStack;

pub(crate) struct ObjFactory;

impl ObjFactory {
    pub(crate) fn create<T>(obj: T) -> *mut T {
        let mut boxed = Box::new(obj);
        let ptr = boxed.as_mut_ptr();

        // TODO: We should somehow register the objects in the garbage collector.
        std::mem::forget(boxed);
        ptr
    }
}

pub(crate) trait VmOperation {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError>;
}

pub struct Vm {
    frame_has_changed: bool,
    chunk: Chunk,
    pub(crate) frame_stack: CallFrameStack,
    pub(crate) scope_stack: ScopeStack,
    pub(crate) const_stack: ConstStack,
    pub(crate) obj_stack: ObjStack,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            frame_has_changed: false,
            chunk,
            frame_stack: CallFrameStack::new(),
            scope_stack: ScopeStack::new(),
            const_stack: ConstStack::new(),
            obj_stack: ObjStack::new(),
        }
    }

    pub fn set_frame_has_changed(&mut self, frame_has_changed: bool) {
        self.frame_has_changed = frame_has_changed;
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let root_fun = ObjFactory::create(ObjFunc::root(self.chunk.clone()));
        let root_frame = CallFrame::new(root_fun, HashMap::new());

        self.frame_stack.push(root_frame);

        self.register_base_classes()?;
        self.register_base_functions()?;
        self.register_base_methods()?;

        while let Some(inst) = self.frame_stack.current().and_then(|f| f.peek()) {
            match inst {
                Bytecode::LoadConstant(i) => LoadConstant::exec(*i, self)?,
                Bytecode::BeginScope => BeginScope::exec(self)?,
                Bytecode::EndScope => EndScope::exec(self)?,
                Bytecode::Lit => Lit::exec(self)?,
                Bytecode::Pop => Pop::exec(self)?,
                Bytecode::ImplTrait => ImplTrait::exec(self)?,
                Bytecode::DeclareTrait => DeclareTrait::exec(self)?,
                Bytecode::DeclareFunc => DeclareFunc::exec(self)?,
                Bytecode::DeclareMethod => DeclareMethod::exec(self)?,
                Bytecode::InstantiateClass => {
                    let class_name = self.const_stack.pop()?.as_str();
                    let class_ptr = self.scope_stack.get(&class_name)?.as_class()?;

                    let mut props = HashMap::new();

                    unsafe {
                        let obj_class = &*class_ptr;

                        // REVIEW: I don't like this strategy, we're creating the fields based on their count.
                        for _ in 0..obj_class.fields_count {
                            let field_name = self.const_stack.pop()?.as_str();
                            let field_value = self.obj_stack.pop()?;
                            props.insert(field_name, field_value);
                        }
                    }

                    let obj_inst = ObjInst::new(class_ptr, props);
                    self.obj_stack.push(Obj::Inst(ObjFactory::create(obj_inst)));
                }
                Bytecode::DeclareVar => {
                    let var_name = self.const_stack.pop()?.as_str();
                    let obj = self.obj_stack.pop()?;

                    self.scope_stack.insert(&var_name, obj);

                    let nil_class_ptr = self.scope_stack.get("Nil")?.as_class()?;

                    self.obj_stack
                        .push(Obj::Prim(ObjFactory::create(ObjPrim::nil(nil_class_ptr))));
                }
                Bytecode::DeclareConst => {
                    let var_name = self.const_stack.pop()?.as_str();
                    let obj = self.obj_stack.pop()?;
                    self.scope_stack.insert(&var_name, obj);
                }
                Bytecode::DeclareClass => {
                    let fields_count = self.const_stack.pop()?.as_num();
                    let class_name = self.const_stack.pop()?.as_str();

                    let class_ptr =
                        ObjFactory::create(ObjClass::new(&class_name, fields_count as u32));

                    self.scope_stack.insert(&class_name, Obj::Class(class_ptr));
                }
                Bytecode::SetProp => {
                    let instance = self.obj_stack.pop()?.as_instance();
                    let prop_name = self.const_stack.pop()?.as_str();
                    let obj = self.obj_stack.pop()?;

                    unsafe {
                        let instance = &mut *instance;
                        instance.set_prop(&prop_name, obj);
                    }

                    let nil_class_ptr = self.scope_stack.get("Nil")?.as_class()?;

                    self.obj_stack
                        .push(Obj::Prim(ObjFactory::create(ObjPrim::nil(nil_class_ptr))));
                }
                Bytecode::GetVar => {
                    let var_name = self.const_stack.pop()?.as_str();
                    let obj = self.scope_stack.get(&var_name)?;
                    self.obj_stack.push(obj);
                }
                Bytecode::GetConst => {
                    let var_name = self.const_stack.pop()?.as_str();
                    let obj = self.scope_stack.get(&var_name)?;
                    self.obj_stack.push(obj);
                }
                Bytecode::GetProp => {
                    let prop_name = self.const_stack.pop()?.as_str();
                    let obj = self.obj_stack.pop()?;

                    let class_ptr = match obj {
                        Obj::Inst(inst) => unsafe { (&*inst).class_ptr() },
                        Obj::Prim(prim) => unsafe { (&*prim).class_ptr() },
                        _ => unreachable!(),
                    };

                    if let Ok(method) = self.scope_stack.method(class_ptr, &prop_name) {
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
                    let obj = self.obj_stack.pop()?;
                    self.const_stack.pop()?.as_num();

                    let mut slots = HashMap::new();

                    match obj {
                        Obj::Func(func) => {
                            unsafe {
                                let func = &*func;
                                for i in 0..func.params.len() {
                                    slots.insert(func.params[i].clone(), self.obj_stack.pop()?);
                                }
                            }

                            self.frame_stack.push(CallFrame::new(func, slots));
                            self.frame_has_changed = true;
                        }
                        Obj::NativeFunc(native_func) => unsafe {
                            let native_func = &*native_func;
                            self.obj_stack.push((native_func.func)(slots)?);
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
                                        slots.insert(func.params[i].clone(), self.obj_stack.pop()?);
                                    }

                                    self.frame_stack.push(CallFrame::new(func_ptr, slots));
                                    self.frame_has_changed = true;
                                }
                                ObjBoundMethodFunc::Native(native_func_ptr) => {
                                    let native_func = &*native_func_ptr;
                                    self.obj_stack.push((native_func.func)(slots)?);
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
                Bytecode::Add => Add::exec(self)?,
                Bytecode::Subtract => Subtract::exec(self)?,
                Bytecode::Println => {
                    let obj = self.obj_stack.pop()?;
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
                _ => {}
            }

            if self.frame_has_changed {
                self.frame_has_changed = false;
            } else {
                self.frame_stack.next();
            }
        }

        // println!("{:#?}", self.scope_stack);

        Ok(())
    }

    fn register_base_classes(&mut self) -> Result<(), RuntimeError> {
        let nil_class_ptr = ObjFactory::create(ObjClass::new("Nil", 0));
        let bool_class_ptr = ObjFactory::create(ObjClass::new("Bool", 0));
        let num_class_ptr = ObjFactory::create(ObjClass::new("Num", 0));

        self.scope_stack.insert("Nil", Obj::Class(nil_class_ptr));
        self.scope_stack.insert("Bool", Obj::Class(bool_class_ptr));
        self.scope_stack.insert("Num", Obj::Class(num_class_ptr));

        Ok(())
    }

    fn register_base_functions(&mut self) -> Result<(), RuntimeError> {
        let start_time = Instant::now();
        let num_class_ptr = self.scope_stack.get("Num")?.as_class()?;

        let clock_native_func = ObjFactory::create(ObjNativeFunc::new(
            "clock",
            move |_| -> Result<Obj, RuntimeError> {
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;

                let res = ObjFactory::create(ObjPrim::num(
                    num_class_ptr,
                    elapsed_time.as_millis() as f64,
                ));
                Ok(Obj::Prim(res))
            },
        ));

        self.scope_stack
            .insert("clock", Obj::NativeFunc(clock_native_func));

        Ok(())
    }

    fn register_base_methods(&mut self) -> Result<(), RuntimeError> {
        self.define_method(
            "Num",
            "add",
            move |cls_ptr, params| -> Result<Obj, RuntimeError> {
                let this = params
                    .get("this")
                    .ok_or(RuntimeError::new("Symbol 'this' not found in this scope"))?
                    .as_prim()?;

                let other = params
                    .get("other")
                    .ok_or(RuntimeError::new("Symbol 'other' not found in this scope"))?
                    .as_prim()?;

                let result = unsafe {
                    let this = &*this;
                    let other = &*other;
                    ObjPrim::num(cls_ptr, this.value + other.value)
                };

                Ok(Obj::Prim(ObjFactory::create(result)))
            },
        )?;

        let num_class_ptr = self.scope_stack.get("Num")?.as_class()?;

        let num_add_method = ObjFactory::create(ObjNativeFunc::new(
            "add",
            move |params| -> Result<Obj, RuntimeError> {
                let this = params
                    .get("this")
                    .ok_or(RuntimeError::new("Symbol 'this' not found in this scope"))?
                    .as_prim()?;

                let other = params
                    .get("other")
                    .ok_or(RuntimeError::new("Symbol 'other' not found in this scope"))?
                    .as_prim()?;

                unsafe {
                    let this = &*this;
                    let other = &*other;

                    let res =
                        ObjFactory::create(ObjPrim::num(num_class_ptr, this.value + other.value));
                    Ok(Obj::Prim(res))
                }
            },
        ));

        let num_sub_method = ObjFactory::create(ObjNativeFunc::new(
            "sub",
            move |params| -> Result<Obj, RuntimeError> {
                let this = params
                    .get("this")
                    .ok_or(RuntimeError::new("Symbol 'this' not found in this scope"))?
                    .as_prim()?;

                let other = params
                    .get("other")
                    .ok_or(RuntimeError::new("Symbol 'other' not found in this scope"))?
                    .as_prim()?;

                unsafe {
                    let this = &*this;
                    let other = &*other;

                    let res =
                        ObjFactory::create(ObjPrim::num(num_class_ptr, this.value - other.value));
                    Ok(Obj::Prim(res))
                }
            },
        ));

        let num_copy = ObjFactory::create(ObjNativeFunc::new(
            "copy",
            move |params| -> Result<Obj, RuntimeError> {
                let this = params
                    .get("this")
                    .ok_or(RuntimeError::new("Symbol 'this' not found in this scope"))?
                    .as_prim()?;

                unsafe {
                    let this = &*this;

                    let res = ObjFactory::create(ObjPrim::num(num_class_ptr, this.value));
                    Ok(Obj::Prim(res))
                }
            },
        ));

        self.scope_stack
            .set_method(num_class_ptr, "add", Obj::NativeFunc(num_add_method));
        self.scope_stack
            .set_method(num_class_ptr, "sub", Obj::NativeFunc(num_sub_method));
        self.scope_stack
            .set_method(num_class_ptr, "copy", Obj::NativeFunc(num_copy));

        Ok(())
    }

    pub(crate) fn create_nil(&mut self, b: bool) -> Result<Obj, RuntimeError> {
        let cls_ptr = self.scope_stack.get("Nil")?.as_class()?;
        let res = ObjFactory::create(ObjPrim::nil(cls_ptr));
        Ok(Obj::Prim(res))
    }

    pub(crate) fn create_bool(&mut self, b: bool) -> Result<Obj, RuntimeError> {
        let cls_ptr = self.scope_stack.get("Bool")?.as_class()?;
        let res = ObjFactory::create(ObjPrim::bool(cls_ptr, b));
        Ok(Obj::Prim(res))
    }

    pub(crate) fn create_number(&mut self, n: f64) -> Result<Obj, RuntimeError> {
        let cls_ptr = self.scope_stack.get("Num")?.as_class()?;
        let res = ObjFactory::create(ObjPrim::num(cls_ptr, n));
        Ok(Obj::Prim(res))
    }

    pub(crate) fn define_func<F>(&mut self, func_name: &str, f: F)
    where
        F: Fn(HashMap<String, Obj>) -> Result<Obj, RuntimeError> + 'static,
    {
        let native_func_ptr = ObjFactory::create(ObjNativeFunc::new(func_name, f));
        self.scope_stack
            .insert(func_name, Obj::NativeFunc(native_func_ptr));
    }

    pub(crate) fn define_method<F>(
        &mut self,
        cls_name: &str,
        method_name: &str,
        f: F,
    ) -> Result<(), RuntimeError>
    where
        F: Fn(*mut ObjClass, HashMap<String, Obj>) -> Result<Obj, RuntimeError> + 'static,
    {
        let cls_ptr = self.scope_stack.get(cls_name)?.as_class()?;
        let native_func = ObjNativeFunc::new(method_name, move |params| f(cls_ptr, params));
        let native_func_ptr = ObjFactory::create(native_func);
        let obj = Obj::NativeFunc(native_func_ptr);

        self.scope_stack.set_method(cls_ptr, method_name, obj);

        Ok(())
    }
}
