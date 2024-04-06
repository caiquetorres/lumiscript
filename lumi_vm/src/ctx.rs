use std::collections::HashMap;
use std::rc::Rc;

use lumi_bc_e::chunk::{Bytecode, Chunk, Constant};

use crate::const_stack::ConstStack;
use crate::frame::{CallFrame, CallStack};
use crate::obj_stack::ObjectStack;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::scope::Scope;

pub(crate) struct ExecutionContext {
    pub(crate) chunk: Chunk,
    constant_stack: ConstStack,
    object_stack: ObjectStack,
    pub(crate) call_stack: CallStack,
    pub(crate) scope: Rc<Scope>,
}

impl ExecutionContext {
    pub(crate) fn new(chunk: Chunk) -> Self {
        let mut call_stack = CallStack::new();
        let root_scope = Rc::new(Scope::root());
        call_stack.push(CallFrame::new(
            Rc::clone(&root_scope),
            0,
            chunk.len() - 1,
            HashMap::new(),
            None,
        ));
        Self {
            chunk,
            constant_stack: ConstStack::new(),
            object_stack: ObjectStack::new(),
            call_stack,
            scope: root_scope,
        }
    }
}

impl ExecutionContext {
    pub(crate) fn next_bytecode(&mut self) -> Option<(&CallFrame, Bytecode, usize)> {
        self.call_stack.current_mut().and_then(|frame| {
            let bytecode_position = frame.start + frame.index;
            self.chunk
                .bytecode(bytecode_position)
                .map(|bytecode| (&*frame, bytecode, bytecode_position))
        })
    }

    pub(crate) fn add_scope(&mut self, slots: HashMap<String, Object>) {
        self.scope = Rc::new(Scope::new(Rc::clone(&self.scope)));
        for (key, object) in &slots {
            self.scope.set_symbol(key, object.clone());
        }
    }

    pub(crate) fn drop_scope(&mut self) {
        if let Some(parent) = &self.scope.parent {
            self.scope = Rc::clone(&parent);
        }
    }

    pub(crate) fn push_frame(&mut self, frame: CallFrame) {
        self.call_stack.push(frame)
    }

    pub(crate) fn pop_frame(&mut self) {
        // REVIEW: Improve this please
        self.scope = self.call_stack.current().unwrap().root_scope.clone();
        self.call_stack.pop()
    }

    pub(crate) fn push_constant(&mut self, constant: Constant) {
        self.constant_stack.push(constant)
    }

    pub(crate) fn pop_constant(&mut self) -> Constant {
        self.constant_stack.pop()
    }

    pub(crate) fn push_object(&mut self, object: Object) {
        self.object_stack.push(object)
    }

    pub(crate) fn pop_object(&mut self) -> Object {
        self.object_stack.pop()
    }

    pub(crate) fn set_instruction(&mut self, instruction_position: usize) {
        if let Some(frame) = self.call_stack.current_mut() {
            frame.index = instruction_position;
        }
    }

    pub(crate) fn next_instruction(&mut self) {
        if let Some(frame) = self.call_stack.current_mut() {
            frame.index += 1;
        }
    }

    pub(crate) fn next_n_instruction(&mut self, n: usize) {
        if let Some(frame) = self.call_stack.current_mut() {
            frame.index += n;
        }
    }
}
