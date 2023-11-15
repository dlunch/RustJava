use classfile::Opcode;

use crate::{stack_frame::StackFrame, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(_stack_frame: &StackFrame, _bytecode: &[Opcode]) -> JvmResult<()> {
        Ok(())
    }
}
