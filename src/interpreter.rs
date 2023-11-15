use classfile::{ConstantPoolItem, Opcode};

use crate::{stack_frame::StackFrame, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(_stack_frame: &StackFrame, _constant_pool: &[ConstantPoolItem], bytecode: &[Opcode]) -> JvmResult<()> {
        for opcode in bytecode {
            match opcode {
                Opcode::Ldc(_) => {}
                Opcode::Getstatic(_) => {}
                Opcode::Invokevirtual(_) => {}
                Opcode::Return => {}
                _ => panic!("Unimplemented opcode {:?}", opcode),
            }
        }
        Ok(())
    }
}
