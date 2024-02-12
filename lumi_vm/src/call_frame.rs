use std::collections::HashMap;

use compiler::generator::bytecode::Bytecode;
use compiler::generator::constant::Constant;

use crate::obj::Obj;
use crate::obj::ObjFunc;

/// Represents a call frame used in the execution of the virtual machine.
#[derive(Debug)]
pub(crate) struct CallFrame {
    /// The instruction pointer, indicating the position of the next
    /// bytecode instruction to be executed.
    ip: usize,

    /// A raw pointer to the associated `ObjectFunction`.
    function: *mut ObjFunc,

    slots: HashMap<String, Obj>,
}

impl CallFrame {
    pub(crate) fn new(function: *mut ObjFunc, slots: HashMap<String, Obj>) -> Self {
        Self {
            ip: 0,
            function,
            slots,
        }
    }

    /// Retrieves the next bytecode instruction from the associated
    /// function's chunk without advancing the instruction pointer.
    ///
    /// # Returns
    /// An `Option` containing the next `Bytecode` instruction if
    /// available, or `None` if the end of the chunk is reached.
    pub(crate) fn peek(&self) -> Option<&Bytecode> {
        self.function().chunk.buffer().get(self.ip)
    }

    pub(crate) fn constant(&self, pos: usize) -> Option<&Constant> {
        self.function().chunk.constant(pos)
    }

    pub(crate) fn function(&self) -> &ObjFunc {
        unsafe { &*self.function }
    }

    pub(crate) fn slots(&self) -> &HashMap<String, Obj> {
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

    pub(crate) fn pop(&mut self) {
        self.frames.pop();
    }

    pub(crate) fn next(&mut self) {
        if let Some(current) = self.frames.last_mut() {
            current.ip += 1;
        }
    }

    pub(crate) fn push(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }
}
