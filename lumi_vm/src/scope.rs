use std::collections::HashMap;
use std::collections::HashSet;

use crate::obj::Obj;
use crate::obj::ObjClass;
use crate::obj::ObjTrait;
use crate::runtime_error::RuntimeError;

/// The `Scope` struct is designed to serve as a container for storing
/// symbols actively in use within a specific scope. This allows for the
/// creation of references to these symbols.

/// For instance, when a variable like `x` is declared and needs to be
/// inserted into the scope, it becomes accessible within that scope and
/// any of its child scopes.
#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Obj>,
    traits: HashSet<(*mut ObjClass, *mut ObjTrait)>,
    methods: HashMap<(*mut ObjClass, String), Obj>,
}

impl Scope {
    fn new() -> Self {
        Self {
            traits: HashSet::new(),
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
    /// Constructs a new instance of the stack.
    ///
    /// The stack is initialized with a root scope by default.
    ///
    /// # Returns
    /// A new `ScopeStack` instance with an initial root scope.
    pub(crate) fn new() -> Self {
        let root_scope = Scope::new();
        Self {
            buffer: vec![root_scope],
        }
    }

    /// Adds a new empty scope to the top of the stack.
    pub(crate) fn push(&mut self) {
        let scope = Scope::new();
        self.buffer.push(scope);
    }

    /// Removes the top scope from the stack.
    pub(crate) fn pop(&mut self) {
        self.buffer.pop();
    }

    /// Inserts a new symbol in the stack top scope.
    ///
    /// # Parameters
    /// - `ident`: A string representing the identifier of the object to be
    /// retrieved.
    /// - `object`: The object that is being inserted.
    pub(crate) fn insert(&mut self, ident: &str, object: Obj) {
        if let Some(current) = self.current_mut() {
            current.symbols.insert(ident.to_owned(), object);
        }
    }

    /// Retrieves the object associated with the specified identifier by
    /// searching through scopes from the top and moving to the previous
    /// ones.
    ///
    /// # Parameters
    /// - `ident`: A string representing the identifier of the object to be
    /// retrieved.
    ///
    /// # Returns
    /// An `Option` containing the retrieved `Object` if the identifier is
    /// found, or `None` if the identifier is not present in any of the
    /// scopes.
    pub(crate) fn get(&self, ident: &str) -> Result<Obj, RuntimeError> {
        for current in self.buffer.iter().rev() {
            if let Some(obj) = current.symbols.get(ident) {
                return Ok(obj.clone());
            }
        }
        Err(RuntimeError::new("Identifier not found"))
    }

    pub fn set_method(&mut self, cls: *mut ObjClass, ident: &str, method: Obj) {
        if let Some(current) = self.current_mut() {
            current.methods.insert((cls, ident.to_owned()), method);
        }
    }

    pub fn method(&self, cls: *mut ObjClass, ident: &str) -> Result<Obj, RuntimeError> {
        for current in self.buffer.iter().rev() {
            if let Some(obj) = current.methods.get(&(cls, ident.to_owned())) {
                return Ok(obj.clone());
            }
        }
        Err(RuntimeError::new("Method not implemented for class"))
    }

    pub(crate) fn has_impl(&self, cls: *mut ObjClass, tr: *mut ObjTrait) -> bool {
        self.buffer
            .iter()
            .rev()
            .any(|scope| scope.traits.contains(&(cls, tr)))
    }

    pub(crate) fn set_impl(&mut self, cls: *mut ObjClass, tr: *mut ObjTrait) {
        if let Some(current) = self.current_mut() {
            current.traits.insert((cls, tr));
        }
    }

    fn current_mut(&mut self) -> Option<&mut Scope> {
        self.buffer.last_mut()
    }
}
