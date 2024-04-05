use std::{collections::HashMap, rc::Rc};

use crate::{runtime_error::RuntimeError, scope::Scope};

pub(crate) trait FromPtr<T> {
    fn from_ptr(&self) -> &T;
}

pub(crate) trait FromMut<T> {
    fn from_mut(&self) -> &mut T;
}

pub(crate) trait ToPtr<T> {
    fn to_ptr(&self) -> *const T;

    fn to_mut(&mut self) -> *mut T;
}

impl<T> ToPtr<T> for Box<T> {
    fn to_ptr(&self) -> *const T {
        self.as_ref() as *const T
    }

    fn to_mut(&mut self) -> *mut T {
        self.as_mut() as *mut T
    }
}

impl<T> FromPtr<T> for *const T {
    fn from_ptr(&self) -> &T {
        unsafe { &*(*self) }
    }
}

impl<T> FromMut<T> for *mut T {
    fn from_mut(&self) -> &mut T {
        unsafe { &mut *(*self) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Class(*const Class),
    Primitive(*const Primitive),
    Instance(*mut Instance),
    Function(*const Function),
    Method(*const Method),
}

impl Object {
    pub(crate) fn class_ptr(&self) -> Option<*const Class> {
        match self {
            Self::Instance(instance) => Some(instance.from_mut().class),
            Self::Primitive(primitive) => Some(primitive.from_ptr().class),
            _ => None,
        }
    }
}

pub(crate) struct Class {
    name: String,
}

impl Class {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Primitive {
    class: *const Class,
    value: f64,
}

impl Primitive {
    pub(crate) fn new(class: *const Class, value: f64) -> Self {
        Self { class, value }
    }

    pub(crate) fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Instance {
    class: *const Class,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(class: *const Class, fields: HashMap<String, Object>) -> Self {
        Self { class, fields }
    }

    pub fn class(&self) -> &Class {
        self.class.from_ptr()
    }

    pub fn field(&self, ident: &str) -> Option<&Object> {
        self.fields.get(ident)
    }

    pub fn set_field(&mut self, ident: &str, value: Object) {
        self.fields.insert(ident.to_owned(), value);
    }
}

pub(crate) struct Method {
    this: Object,
    function: *const Function,
}

impl Method {
    pub(crate) fn new(this: Object, function: *const Function) -> Self {
        Self { this, function }
    }

    pub(crate) fn this(&self) -> &Object {
        &self.this
    }

    pub(crate) fn function(&self) -> &Function {
        self.function.from_ptr()
    }
}

pub(crate) enum Function {
    Default {
        scope: Rc<Scope>,
        name: String,
        params: Vec<String>,
        start: usize,
        end: usize,
    },
    Native {
        name: String,
        params: Vec<String>,
        fun: Box<dyn Fn(HashMap<String, Object>) -> Result<Object, RuntimeError>>,
    },
}

impl Function {
    pub(crate) fn default(
        scope: Rc<Scope>,
        name: &str,
        start: usize,
        end: usize,
        params: &[String],
    ) -> Self {
        Self::Default {
            scope,
            name: name.to_owned(),
            params: params.to_vec(),
            start,
            end,
        }
    }

    pub(crate) fn native<F>(name: &str, params: &[String], fun: F) -> Self
    where
        F: 'static + Fn(HashMap<String, Object>) -> Result<Object, RuntimeError>,
    {
        Self::Native {
            name: name.to_owned(),
            params: params.to_vec(),
            fun: Box::new(fun),
        }
    }

    pub(crate) fn name(&self) -> String {
        match self {
            Self::Native { name, .. } => name.clone(),
            Self::Default { name, .. } => name.clone(),
        }
    }

    pub(crate) fn params(&self) -> &Vec<String> {
        match self {
            Self::Native { params, .. } => params,
            Self::Default { params, .. } => params,
        }
    }
}
