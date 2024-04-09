use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct InnerScope {
    symbols: HashMap<String, usize>,
    methods: HashMap<(usize, String), usize>,
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

    pub(crate) fn set_symbol(&self, ident: &str, object: usize) {
        self.inner
            .borrow_mut()
            .symbols
            .insert(ident.to_owned(), object);
    }

    pub(crate) fn assign_symbol(&self, ident: &str, object: usize) -> Option<()> {
        if let Some(symbol) = self.inner.borrow_mut().symbols.get_mut(ident) {
            *symbol = object;
            Some(())
        } else {
            if let Some(parent) = &self.parent {
                parent.assign_symbol(ident, object)
            } else {
                None
            }
        }
    }

    pub(crate) fn symbol(&self, ident: &str) -> Option<usize> {
        if let Some(value) = self.inner.borrow().symbols.get(ident) {
            Some(*value)
        } else {
            if let Some(parent) = &self.parent {
                parent.symbol(ident)
            } else {
                None
            }
        }
    }

    pub(crate) fn set_method(&self, class: usize, ident: &str, method: usize) {
        self.inner
            .borrow_mut()
            .methods
            .insert((class, ident.to_owned()), method);
    }

    pub(crate) fn method(&self, class: usize, ident: &str) -> Option<usize> {
        if let Some(value) = self.inner.borrow().methods.get(&(class, ident.to_owned())) {
            Some(value.clone())
        } else {
            if let Some(parent) = &self.parent {
                parent.method(class, ident)
            } else {
                None
            }
        }
    }
}
