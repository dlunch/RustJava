use classfile::{ConstantPoolItem, Opcode};

use crate::{value::JavaValue, Jvm, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(jvm: &mut Jvm, _constant_pool: &[ConstantPoolItem], bytecode: &[Opcode]) -> JvmResult<JavaValue> {
        let _stack_frame = jvm.current_thread_context().current_stack_frame();

        for opcode in bytecode {
            match opcode {
                Opcode::Ldc(_) => {}
                Opcode::Getstatic(_) => {}
                Opcode::Invokevirtual(_) => {}
                Opcode::Return => {
                    return Ok(JavaValue::Void);
                }
                _ => panic!("Unimplemented opcode {:?}", opcode),
            }
        }

        panic!("Should not reach here")
    }
}
