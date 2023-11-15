use alloc::vec::Vec;

pub enum StackVariable {}

pub struct StackFrame {
    pub local_variables: Vec<StackVariable>,
    pub operand_stack: Vec<StackVariable>,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            local_variables: Vec::new(),
            operand_stack: Vec::new(),
        }
    }
}
