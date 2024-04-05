use std::{collections::HashMap, rc::Rc};

use crate::{object::Object, scope::Scope};

/// Represents a call frame used in the execution of the virtual machine.
#[derive(Debug)]
pub(crate) struct CallFrame {
    pub(crate) root_scope: Rc<Scope>,
    pub(crate) index: usize,
    pub(crate) start: usize,
    _end: usize,
    slots: HashMap<String, Object>,
}

impl CallFrame {
    pub(crate) fn new(
        scope: Rc<Scope>,
        start: usize,
        end: usize,
        slots: HashMap<String, Object>,
    ) -> Self {
        Self {
            root_scope: scope,
            index: 0,
            start,
            _end: end,
            slots,
        }
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

    pub(crate) fn move_ptr(&mut self, offset: usize) {
        if let Some(frame) = self.frames.last_mut() {
            frame.index += offset;
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
