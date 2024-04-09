use std::{collections::HashMap, fmt::Debug, ops::Range, rc::Rc};

use crate::{runtime_error::RuntimeError, scope::Scope, vm::Vm};

#[derive(Debug)]
pub(crate) enum Object {
    Class(Class),
    Primitive(Primitive),
    Instance(Instance),
    Function(Function),
}

impl Object {
    pub(crate) fn class_id(&self) -> Option<usize> {
        match self {
            Self::Class(_) => None,
            Self::Function(_) => None,
            Self::Instance(instance) => Some(instance.class),
            Self::Primitive(primitive) => Some(primitive.class),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct Instance {
    class: usize,
    fields: HashMap<String, usize>,
}

impl Instance {
    pub(crate) fn new(class: usize, fields: HashMap<String, usize>) -> Self {
        Self { class, fields }
    }

    pub(crate) fn class_id(&self) -> usize {
        self.class
    }

    pub(crate) fn field(&self, key: &str) -> Option<usize> {
        self.fields.get(key).map(|value| *value)
    }

    pub(crate) fn set_field(&mut self, key: &str, value: usize) {
        self.fields.insert(key.to_owned(), value);
    }
}

#[derive(Debug)]
pub(crate) struct Primitive {
    class: usize,
    value: f64,
}

impl Primitive {
    pub(crate) fn new(class: usize, value: f64) -> Self {
        Self { class, value }
    }

    pub(crate) fn class(&self) -> usize {
        self.class
    }

    pub(crate) fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug)]
pub(crate) struct Function {
    name: String,
    params: Vec<String>,
    class: Option<usize>,
    inner: InnerFunction,
}

pub(crate) type NativeFunction =
    Box<dyn Fn(&Vm, HashMap<String, usize>) -> Result<Object, RuntimeError>>;

pub(crate) enum InnerFunction {
    Native {
        fun: NativeFunction,
    },
    Frame {
        scope: Rc<Scope>,
        range: Range<usize>,
    },
}

impl Debug for InnerFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native { .. } => f.debug_struct("Native").finish(),
            Self::Frame { scope, range } => f
                .debug_struct("Frame")
                .field("scope", scope)
                .field("range", range)
                .finish(),
        }
    }
}

impl Function {
    pub(crate) fn new(
        name: &str,
        params: &[String],
        class: Option<usize>,
        inner: InnerFunction,
    ) -> Self {
        Self {
            name: name.to_owned(),
            params: params.to_vec(),
            class,
            inner,
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn params(&self) -> &Vec<String> {
        &self.params
    }

    pub(crate) fn inner(&self) -> &InnerFunction {
        &self.inner
    }

    pub(crate) fn class(&self) -> Option<usize> {
        self.class
    }
}

impl InnerFunction {
    pub(crate) fn frame(scope: Rc<Scope>, range: Range<usize>) -> Self {
        Self::Frame { scope, range }
    }
}
