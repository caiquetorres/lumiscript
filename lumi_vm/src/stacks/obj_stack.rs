use crate::obj::Obj;

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

    pub(crate) fn pop(&mut self) -> Obj {
        self.buffer.pop().unwrap()
    }
}
