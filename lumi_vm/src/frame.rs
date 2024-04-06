use std::{collections::HashMap, rc::Rc};

use lumi_lxr::span::Span;

use crate::object::Object;
use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct Trace {
    pub(crate) class_name: Option<String>,
    pub(crate) function_name: String,
    pub(crate) span: Span,
}

impl Trace {
    pub(crate) fn for_function(function_name: &str, span: Span) -> Self {
        Self {
            class_name: None,
            function_name: function_name.to_owned(),
            span,
        }
    }

    pub(crate) fn for_method(class_name: &str, function_name: &str, span: Span) -> Self {
        Self {
            class_name: Some(class_name.to_owned()),
            function_name: function_name.to_owned(),
            span,
        }
    }
}

/// Represents a call frame used in the execution of the virtual machine.
#[derive(Debug)]
pub(crate) struct CallFrame {
    pub(crate) root_scope: Rc<Scope>,
    pub(crate) index: usize,
    pub(crate) start: usize,
    _end: usize,
    slots: HashMap<String, Object>,
    trace: Option<Trace>,
}

impl CallFrame {
    pub(crate) fn new(
        scope: Rc<Scope>,
        start: usize,
        end: usize,
        slots: HashMap<String, Object>,
        trace: Option<Trace>,
    ) -> Self {
        Self {
            root_scope: scope,
            index: 0,
            start,
            _end: end,
            slots,
            trace,
        }
    }

    pub(crate) fn slots(&self) -> &HashMap<String, Object> {
        &self.slots
    }
}

#[derive(Debug)]
pub(crate) struct CallStack {
    frames: Vec<CallFrame>,
}

impl CallStack {
    pub(crate) fn new() -> Self {
        Self { frames: vec![] }
    }

    pub(crate) fn current(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    pub(crate) fn current_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    pub(crate) fn pop(&mut self) {
        self.frames.pop();
    }

    pub(crate) fn push(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }

    pub(crate) fn stack_trace(&self) -> Vec<Trace> {
        self.frames
            .iter()
            .map(|frame| frame.trace.clone())
            .flatten()
            .collect()
    }
}
