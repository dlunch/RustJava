use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use jvm::ThreadContext;

use crate::stack_frame::StackFrame;

#[derive(Default)]
pub struct ThreadContextImpl {
    stack: Vec<Rc<RefCell<StackFrame>>>,
}

impl ThreadContextImpl {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_stack_frame(&mut self) -> Rc<RefCell<StackFrame>> {
        let value = Rc::new(RefCell::new(StackFrame::new()));
        self.stack.push(value.clone());

        value
    }
}

impl ThreadContext for ThreadContextImpl {}
