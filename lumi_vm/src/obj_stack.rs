use crate::{runtime_error::RuntimeError, Object};

#[derive(Debug)]
pub(crate) struct ObjectStack {
    buffer: Vec<Object>,
}

impl ObjectStack {
    pub(crate) fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(&mut self, obj: Object) {
        self.buffer.push(obj)
    }

    pub(crate) fn pop(&mut self) -> Result<Object, RuntimeError> {
        self.buffer
            .pop()
            .ok_or(RuntimeError::new("Object stack underflow"))
    }

    pub(crate) fn peek(&mut self) -> Result<&Object, RuntimeError> {
        self.buffer
            .last()
            .ok_or(RuntimeError::new("Object stack underflow"))
    }
}
