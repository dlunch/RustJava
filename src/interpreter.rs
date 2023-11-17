use alloc::collections::BTreeMap;
use classfile::Opcode;

use crate::{value::JavaValue, Jvm, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(jvm: &mut Jvm, bytecode: &BTreeMap<u32, Opcode>) -> JvmResult<JavaValue> {
        let _stack_frame = jvm.current_thread_context().current_stack_frame();

        let mut iter = bytecode.range(0..);
        while let Some((_, opcode)) = iter.next() {
            match opcode {
                Opcode::Ldc(_) => {}
                Opcode::Getstatic(_) => {}
                Opcode::Invokevirtual(_) => {}
                Opcode::Goto(x) => {
                    iter = bytecode.range(*x as u32..);
                }
                Opcode::Return => {
                    return Ok(JavaValue::Void);
                }
                _ => panic!("Unimplemented opcode {:?}", opcode),
            }
        }

        panic!("Should not reach here")
    }
}
