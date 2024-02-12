use crate::obj::Obj;
use crate::runtime_error::RuntimeError;

#[derive(Debug)]
pub(crate) struct ObjStack {
    buffer: Vec<Obj>,
}

impl ObjStack {
    pub(crate) fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(&mut self, obj: Obj) {
        self.buffer.push(obj)
    }

    pub(crate) fn pop(&mut self) -> Result<Obj, RuntimeError> {
        self.buffer
            .pop()
            .ok_or(RuntimeError::new("Object stack underflow"))
    }

    pub(crate) fn peek(&mut self) -> Result<Obj, RuntimeError> {
        self.buffer
            .last()
            .map(|obj| obj.clone())
            .ok_or(RuntimeError::new("Object stack underflow"))
    }
}
