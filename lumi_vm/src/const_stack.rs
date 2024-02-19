use lumi_bc_e::chunk::Constant;

use crate::runtime_error::RuntimeError;

#[derive(Debug)]
pub(crate) struct ConstStack {
    buffer: Vec<Constant>,
}

impl ConstStack {
    pub(crate) fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(&mut self, constant: Constant) {
        self.buffer.push(constant)
    }

    pub(crate) fn pop(&mut self) -> Result<Constant, RuntimeError> {
        self.buffer
            .pop()
            .ok_or(RuntimeError::new("Constant stack underflow"))
    }
}
