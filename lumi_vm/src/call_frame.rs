use std::collections::HashMap;

use std::rc::Rc;

use crate::scope::Scope;

#[derive(Debug)]
pub(crate) struct CallFrame {
    pub(crate) instructions_ptr: usize,
    return_scope: Option<Rc<Scope>>,
    slots: HashMap<String, usize>,
}

impl CallFrame {
    pub(crate) fn new(return_scope: Option<Rc<Scope>>, slots: HashMap<String, usize>) -> Self {
        Self {
            instructions_ptr: 0,
            return_scope,
            slots,
        }
    }

    pub(crate) fn return_scope(&self) -> Option<Rc<Scope>> {
        self.return_scope.as_ref().map(|scope| Rc::clone(&scope))
    }

    pub(crate) fn slots(&self) -> HashMap<String, usize> {
        self.slots.clone()
    }
}

pub(crate) struct CallStack {
    root: CallFrame,
    stack: Vec<CallFrame>,
}

impl CallStack {
    pub(crate) fn new(root_call_frame: CallFrame) -> Self {
        Self {
            root: root_call_frame,
            stack: vec![],
        }
    }

    pub(crate) fn current(&self) -> &CallFrame {
        self.stack.last().unwrap_or(&self.root)
    }

    pub(crate) fn current_mut(&mut self) -> &mut CallFrame {
        self.stack.last_mut().unwrap_or(&mut self.root)
    }

    pub(crate) fn push(&mut self, frame: CallFrame) {
        self.stack.push(frame)
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }
}
