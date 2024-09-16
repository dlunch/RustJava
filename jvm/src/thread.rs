use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::class_loader::Class;

pub struct JvmThread {
    stack: Vec<JvmStackFrame>,
}

impl JvmThread {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_frame(&mut self, class: &Class, method_name: &str) {
        self.stack.push(JvmStackFrame {
            class: class.clone(),
            method_name: method_name.to_string(),
        });
    }

    pub fn pop_frame(&mut self) -> Option<JvmStackFrame> {
        self.stack.pop()
    }

    pub fn top_frame(&self) -> Option<&JvmStackFrame> {
        self.stack.last()
    }
}

pub struct JvmStackFrame {
    pub class: Class,
    #[allow(dead_code)] // TODO for debug
    pub method_name: String,
}
