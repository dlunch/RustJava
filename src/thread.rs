use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::stack_frame::StackFrame;

pub type ThreadId = usize;

pub struct ThreadContext {
    stack: Vec<Rc<RefCell<StackFrame>>>, // Wrapping StackFrame in Rc to break borrow chain from Jvm
}

impl ThreadContext {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_stack_frame(&mut self) {
        self.stack.push(Rc::new(RefCell::new(StackFrame::new())))
    }

    pub fn current_stack_frame(&self) -> Rc<RefCell<StackFrame>> {
        self.stack[self.stack.len() - 1].clone()
    }
}
