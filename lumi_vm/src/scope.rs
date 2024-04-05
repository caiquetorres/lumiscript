use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::{Class, Function, Object};
use crate::runtime_error::RuntimeError;

#[derive(Debug)]
struct InnerScope {
    symbols: HashMap<String, Object>,
    methods: HashMap<(*const Class, String), *const Function>,
}

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    pub(crate) parent: Option<Rc<Scope>>,
    inner: Rc<RefCell<InnerScope>>,
}

impl Scope {
    pub(crate) fn root() -> Self {
        Self {
            parent: None,
            inner: Rc::new(RefCell::new(InnerScope {
                symbols: HashMap::new(),
                methods: HashMap::new(),
            })),
        }
    }

    pub(crate) fn new(parent: Rc<Scope>) -> Self {
        Self {
            parent: Some(parent),
            inner: Rc::new(RefCell::new(InnerScope {
                symbols: HashMap::new(),
                methods: HashMap::new(),
            })),
        }
    }

    pub(crate) fn set_symbol(&self, ident: &str, object: Object) {
        self.inner
            .borrow_mut()
            .symbols
            .insert(ident.to_owned(), object);
    }

    pub(crate) fn assign_symbol(&self, ident: &str, object: Object) -> Result<(), RuntimeError> {
        if let Some(symbol) = self.inner.borrow_mut().symbols.get_mut(ident) {
            *symbol = object;
            Ok(())
        } else {
            if let Some(parent) = &self.parent {
                parent.assign_symbol(ident, object)
            } else {
                Err(RuntimeError::new(&format!(
                    "Identifier '{}' not found",
                    ident
                )))
            }
        }
    }

    pub(crate) fn symbol(&self, ident: &str) -> Result<Object, RuntimeError> {
        if let Some(value) = self.inner.borrow().symbols.get(ident) {
            Ok(value.clone())
        } else {
            if let Some(parent) = &self.parent {
                parent.symbol(ident)
            } else {
                Err(RuntimeError::new(&format!(
                    "Identifier '{}' not found",
                    ident
                )))
            }
        }
    }

    pub(crate) fn set_method(&self, cls: *const Class, ident: &str, method: *const Function) {
        self.inner
            .borrow_mut()
            .methods
            .insert((cls, ident.to_owned()), method);
    }

    pub(crate) fn method(
        &self,
        cls: *const Class,
        ident: &str,
    ) -> Result<*const Function, RuntimeError> {
        if let Some(value) = self.inner.borrow().methods.get(&(cls, ident.to_owned())) {
            Ok(value.clone())
        } else {
            if let Some(parent) = &self.parent {
                parent.method(cls, ident)
            } else {
                Err(RuntimeError::new(&format!(
                    "Identifier '{}' not found",
                    ident
                )))
            }
        }
    }
}
