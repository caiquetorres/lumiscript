use crate::object::Object;

pub(crate) struct Memory {
    heap: Vec<Object>,
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self { heap: vec![] }
    }

    pub(crate) fn alloc(&mut self, object: Object) -> usize {
        self.heap.push(object);
        self.heap.len() - 1
    }

    pub(crate) fn get(&self, index: usize) -> &Object {
        self.heap.get(index).unwrap()
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> &mut Object {
        self.heap.get_mut(index).unwrap()
    }
}
