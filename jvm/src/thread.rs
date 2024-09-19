use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::{class_loader::Class, ClassInstance};

pub struct JvmThread {
    pub stack: Vec<JvmStackFrame>,
}

impl JvmThread {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_frame(&mut self, class: &Class, class_instance: Option<Box<dyn ClassInstance>>, method: &str) {
        self.stack.push(JvmStackFrame {
            class: class.clone(),
            class_instance,
            method: method.to_string(),
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
    pub class_instance: Option<Box<dyn ClassInstance>>,
    pub method: String,
}
