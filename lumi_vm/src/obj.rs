use std::collections::HashMap;
use std::fmt::Display;

use compiler::generator::chunk::Chunk;

// TODO: We should add the Prim (primitive) variant.

#[derive(Debug, Clone)]
pub enum Obj {
    Prim(*mut ObjPrim),
    Inst(*mut ObjInst),
    Class(*mut ObjClass),
    Func(*mut ObjFunc),
    NativeFunc(*mut ObjNativeFunc),
    BoundMethod(*mut ObjBoundMethod),
}

impl Obj {
    pub fn as_prim(&self) -> *mut ObjPrim {
        if let Obj::Prim(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_instance(&self) -> *mut ObjInst {
        if let Obj::Inst(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_class(&self) -> *mut ObjClass {
        if let Obj::Class(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_function(&self) -> *mut ObjFunc {
        if let Obj::Func(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_native_function(&self) -> *mut ObjNativeFunc {
        if let Obj::NativeFunc(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_bound_method(&self) -> *mut ObjBoundMethod {
        if let Obj::BoundMethod(value) = self {
            *value
        } else {
            panic!();
        }
    }
}

#[derive(Debug)]
pub enum ObjBoundMethodFunc {
    Default(*mut ObjFunc),
    Native(*mut ObjNativeFunc),
}

#[derive(Debug)]
pub struct ObjBoundMethod {
    pub this: Obj,
    pub func: ObjBoundMethodFunc,
}

#[derive(Debug)]
pub struct ObjFunc {
    pub name: String,
    pub params: Vec<String>,
    pub chunk: Chunk,
}

impl ObjFunc {
    pub fn root(chunk: Chunk) -> Self {
        Self {
            chunk,
            name: String::new(),
            params: Vec::new(),
        }
    }
}

pub struct ObjNativeFunc {
    pub name: String,
    pub func: Box<dyn Fn(HashMap<String, Obj>) -> Obj>,
}

impl ObjNativeFunc {
    pub fn new<F: Fn(HashMap<String, Obj>) -> Obj + 'static>(name: &str, func: F) -> Self {
        Self {
            name: name.to_owned(),
            func: Box::new(func),
        }
    }
}

#[derive(Debug)]
pub struct ObjClass {
    pub name: String,
    // TODO: Replace the fields_count for a hash set or a vec.
    pub fields_count: u32,
}

impl ObjClass {
    pub fn new(name: &str, fields_count: u32) -> Self {
        Self {
            name: name.to_owned(),
            fields_count,
        }
    }
}

#[derive(Debug)]
pub struct ObjInst {
    class: *mut ObjClass,
    props: HashMap<String, Obj>,
}

impl ObjInst {
    pub fn new(class: *mut ObjClass, props: HashMap<String, Obj>) -> Self {
        Self { class, props }
    }

    pub fn class_ptr(&self) -> *mut ObjClass {
        *&self.class
    }

    pub fn prop(&self, prop_name: &str) -> Option<Obj> {
        self.props.get(prop_name).map(|obj| obj.clone())
    }

    pub fn set_prop(&mut self, prop_name: &str, prop_value: Obj) {
        self.props.insert(prop_name.to_owned(), prop_value);
    }
}

#[derive(Debug)]
pub enum ObjPrimKind {
    Nil,
    Num,
    Bool,
}

#[derive(Debug)]
pub struct ObjPrim {
    pub class: *mut ObjClass,
    pub value: f64,
    pub kind: ObjPrimKind,
}

impl ObjPrim {
    pub fn num(class: *mut ObjClass, value: f64) -> Self {
        Self {
            class,
            value,
            kind: ObjPrimKind::Num,
        }
    }

    pub fn class_ptr(&self) -> *mut ObjClass {
        *&self.class
    }

    pub fn bool(class: *mut ObjClass, value: bool) -> Self {
        Self {
            class,
            value: if value { 1.0 } else { 0.0 },
            kind: ObjPrimKind::Bool,
        }
    }

    pub fn nil(class: *mut ObjClass) -> Self {
        Self {
            class,
            value: 0.0,
            kind: ObjPrimKind::Nil,
        }
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ObjStack {
    buffer: Vec<Obj>,
}

impl ObjStack {
    pub(crate) fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(&mut self, obj: Obj) {
        self.buffer.push(obj)
    }

    pub(crate) fn pop(&mut self) -> Obj {
        self.buffer.pop().unwrap()
    }
}
