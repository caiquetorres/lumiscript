use std::collections::HashMap;

use crate::object::Class;
use crate::object::Function;
use crate::object::Object;
use crate::runtime_error::RuntimeError;

/// The `Scope` struct is designed to serve as a container for storing
/// symbols actively in use within a specific scope. This allows for the
/// creation of references to these symbols.

/// For instance, when a variable like `x` is declared and needs to be
/// inserted into the scope, it becomes accessible within that scope and
/// any of its child scopes.
#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Object>,
    methods: HashMap<(*const Class, String), *const Function>,
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            methods: HashMap::new(),
        }
    }
}

/// The `ScopeStack` struct represents a stack of scopes, allowing for
/// hierarchical organization of symbol scopes. It is designed to manage
/// the nesting of scopes, enabling storage and retrieval of
/// symbols within different levels of scope.
#[derive(Debug)]
pub(crate) struct ScopeStack {
    buffer: Vec<Scope>,
}

impl ScopeStack {
    pub(crate) fn new() -> Self {
        let root_scope = Scope::new();
        Self {
            buffer: vec![root_scope],
        }
    }

    pub(crate) fn add_scope(&mut self) {
        let scope = Scope::new();
        self.buffer.push(scope);
    }

    pub(crate) fn pop_scope(&mut self) {
        self.buffer.pop();
    }

    pub(crate) fn set_symbol(&mut self, ident: &str, object: Object) {
        if let Some(current) = self.current_mut() {
            current.symbols.insert(ident.to_owned(), object);
        }
    }

    pub(crate) fn symbol(&self, ident: &str) -> Result<Object, RuntimeError> {
        for current in self.buffer.iter().rev() {
            if let Some(obj) = current.symbols.get(ident) {
                return Ok(obj.clone());
            }
        }
        Err(RuntimeError::new(&format!(
            "Identifier '{}' not found",
            ident
        )))
    }

    pub fn set_method(&mut self, cls: *const Class, ident: &str, method: *const Function) {
        if let Some(current) = self.current_mut() {
            current.methods.insert((cls, ident.to_owned()), method);
        }
    }

    pub fn method(&self, cls: *const Class, ident: &str) -> Result<*const Function, RuntimeError> {
        for current in self.buffer.iter().rev() {
            if let Some(obj) = current.methods.get(&(cls, ident.to_owned())) {
                return Ok(obj.clone());
            }
        }
        Err(RuntimeError::new("Method not implemented for class"))
    }

    fn current_mut(&mut self) -> Option<&mut Scope> {
        self.buffer.last_mut()
    }
}
