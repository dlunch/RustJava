use alloc::{vec, vec::Vec};

use jvm::JavaValue;

#[derive(Default)]
pub struct StackFrame {
    pub local_variables: Vec<JavaValue>,
    pub operand_stack: Vec<JavaValue>,
}

impl StackFrame {
    pub fn new(max_locals: usize, max_stack: usize) -> Self {
        Self {
            local_variables: vec![JavaValue::Void; max_locals],
            operand_stack: Vec::with_capacity(max_stack),
        }
    }
}
