use crate::Object;

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

    pub(crate) fn pop(&mut self) -> Object {
        self.buffer.pop().unwrap()
    }
}
