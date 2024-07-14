#![allow(dead_code)] // TODO
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::{class_loader::Class, ClassInstance};

pub struct JvmThread {
    java_thread: Box<dyn ClassInstance>,
    stack: Vec<JvmStackFrame>,
}

impl JvmThread {
    pub fn new(java_thread: Box<dyn ClassInstance>) -> Self {
        Self {
            java_thread,
            stack: Vec::new(),
        }
    }

    pub fn push_frame(&mut self, class: &Class, method_name: &str) {
        self.stack.push(JvmStackFrame::new(class.clone(), method_name));
    }

    pub fn pop_frame(&mut self) -> Option<JvmStackFrame> {
        self.stack.pop()
    }

    pub fn top_frame(&self) -> Option<&JvmStackFrame> {
        self.stack.last()
    }
}

pub struct JvmStackFrame {
    class: Class,
    method_name: String,
}

impl JvmStackFrame {
    pub fn new(class: Class, method_name: &str) -> Self {
        Self {
            class,
            method_name: method_name.to_string(),
        }
    }
}
