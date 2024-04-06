use lumi_bc_e::chunk::Constant;

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

    pub(crate) fn pop(&mut self) -> Constant {
        self.buffer.pop().unwrap()
    }
}
