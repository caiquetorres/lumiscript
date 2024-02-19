use std::collections::HashMap;

use crate::object::Object;

/// Represents a call frame used in the execution of the virtual machine.
#[derive(Debug)]
pub(crate) struct CallFrame {
    pub(crate) index: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
    slots: HashMap<String, Object>,
}

impl CallFrame {
    pub(crate) fn new(start: usize, end: usize, slots: HashMap<String, Object>) -> Self {
        Self {
            index: 0,
            start,
            end,
            slots,
        }
    }

    fn move_ptr(&mut self, index: usize) {
        self.index += index;
    }

    pub(crate) fn slots(&self) -> &HashMap<String, Object> {
        &self.slots
    }
}

#[derive(Debug)]
pub(crate) struct CallFrameStack {
    frames: Vec<CallFrame>,
}

impl CallFrameStack {
    pub(crate) fn new() -> Self {
        Self { frames: vec![] }
    }

    pub(crate) fn current(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    pub(crate) fn move_ptr(&mut self, index: usize) {
        if let Some(frame) = self.frames.last_mut() {
            frame.index += index;
        }
    }

    pub(crate) fn set_ptr(&mut self, index: usize) {
        if let Some(frame) = self.frames.last_mut() {
            frame.index = index;
        }
    }

    pub(crate) fn pop(&mut self) {
        self.frames.pop();
    }

    pub(crate) fn push(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }
}
