use std::collections::HashMap;
use std::fmt::Display;

use super::chunk::Chunk;

// TODO: We should add the Prim (primitive) variant.

#[derive(Debug, Clone)]
pub enum Obj {
    Str(String),
    Nil,
    Bool(bool),
    Float(f64),
    Instance(*mut ObjInst),
    Class(*mut ObjCls),
    Func(*mut ObjFunc),
    NativeFunc(*mut ObjNativeFunc),
    BoundMethod(*mut ObjBoundMethod),
}

impl Obj {
    pub fn as_bool(&self) -> bool {
        if let Obj::Bool(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_float(&self) -> f64 {
        if let Obj::Float(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_str(&self) -> String {
        if let Obj::Str(value) = self {
            value.clone()
        } else {
            panic!();
        }
    }

    pub fn as_instance(&self) -> *mut ObjInst {
        if let Obj::Instance(value) = self {
            *value
        } else {
            panic!();
        }
    }

    pub fn as_class(&self) -> *mut ObjCls {
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
}

#[derive(Debug)]
pub struct ObjBoundMethod {
    pub this: *mut ObjInst,
    pub func: *mut ObjFunc,
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
    pub func: Box<dyn Fn(usize, Vec<Obj>) -> Obj>,
}

#[derive(Debug)]
pub struct ObjCls {
    pub name: String,
    // TODO: Replace the fields_count for a hash set or a vec.
    pub fields_count: u32,
}

#[derive(Debug)]
pub struct ObjInst {
    class: *mut ObjCls,
    props: HashMap<String, Obj>,
}

impl ObjInst {
    pub fn new(class: *mut ObjCls, props: HashMap<String, Obj>) -> Self {
        Self { class, props }
    }

    pub fn class_ptr(&self) -> *mut ObjCls {
        *&self.class
    }

    pub fn prop(&self, prop_name: &str) -> Option<Obj> {
        self.props.get(prop_name).map(|obj| obj.clone())
    }

    pub fn set_prop(&mut self, prop_name: &str, prop_value: Obj) {
        self.props.insert(prop_name.to_owned(), prop_value);
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
