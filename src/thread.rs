use alloc::vec::Vec;

use crate::stack_frame::StackFrame;

pub type ThreadId = usize;

pub struct ThreadContext {
    stack: Vec<StackFrame>,
}

impl ThreadContext {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_stack_frame(&mut self) {
        self.stack.push(StackFrame::new())
    }

    pub fn current_stack_frame(&self) -> &StackFrame {
        &self.stack[self.stack.len() - 1]
    }
}
