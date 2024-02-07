use compiler::generator::bytecode::Bytecode;
use compiler::generator::obj::Obj;
use compiler::generator::obj::ObjFunc;

/// Represents a call frame used in the execution of the virtual machine.
#[derive(Debug)]
pub(crate) struct CallFrame {
    /// The instruction pointer, indicating the position of the next
    /// bytecode instruction to be executed.
    ip: usize,

    /// A raw pointer to the associated `ObjectFunction`.
    function: *mut ObjFunc,

    _slots: Vec<Obj>,
}

impl CallFrame {
    pub(crate) fn new(function: *mut ObjFunc, slots: Vec<Obj>) -> Self {
        Self {
            ip: 0,
            function,
            _slots: slots,
        }
    }

    /// Retrieves the next bytecode instruction from the associated
    /// function's chunk without advancing the instruction pointer.
    ///
    /// # Returns
    /// An `Option` containing the next `Bytecode` instruction if
    /// available, or `None` if the end of the chunk is reached.
    pub(crate) fn peek(&self) -> Option<Bytecode> {
        self.function()
            .chunk
            .buffer()
            .get(self.ip)
            .map(|bytecode| *bytecode)
    }

    /// Retrieves the `Object` associated with the specified position in
    /// the bytecode chunk of the associated function.
    ///
    /// # Parameters
    /// - `pos`: The object's position/index in the bytecode chunk.
    ///
    /// # Returns
    /// An `Option` containing the retrieved `Object` if available at the
    /// specified position, or `None` if the position is out of bounds.
    pub(crate) fn object(&self, pos: usize) -> Option<Obj> {
        self.function().chunk.object(pos)
    }

    pub(crate) fn function(&self) -> &ObjFunc {
        unsafe { &*self.function }
    }

    pub(crate) fn _slots(&self) -> &Vec<Obj> {
        &self._slots
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
