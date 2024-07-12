use alloc::{boxed::Box, vec::Vec};

use crate::{ClassDefinition, ClassInstance, Method};

#[allow(dead_code)]
pub struct JvmThread {
    java_thread: Box<dyn ClassInstance>,
    stack: JvmStack,
}

impl JvmThread {
    pub fn new(java_thread: Box<dyn ClassInstance>) -> Self {
        Self {
            java_thread,
            stack: JvmStack { stack: Vec::new() },
        }
    }
}

#[allow(dead_code)]
pub struct JvmStack {
    stack: Vec<JvmStackFrame>,
}

impl JvmStack {}
#[allow(dead_code)]
pub struct JvmStackFrame {
    class: Box<dyn ClassDefinition>,
    method: Box<dyn Method>,
}
