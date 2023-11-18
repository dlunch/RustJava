use alloc::collections::BTreeMap;
use classfile::{Opcode, ValueConstant};

use crate::{stack_frame::StackFrame, value::JavaValue, Jvm, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(jvm: &mut Jvm, bytecode: &BTreeMap<u32, Opcode>) -> JvmResult<JavaValue> {
        let mut stack_frame = jvm.current_thread_context().current_stack_frame();

        let mut iter = bytecode.range(0..);
        while let Some((_, opcode)) = iter.next() {
            match opcode {
                Opcode::Ldc(x) => Self::load_constant(&mut stack_frame, x),
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

    fn load_constant(stack_frame: &mut StackFrame, constant: &ValueConstant) {
        match constant {
            ValueConstant::Integer(x) => stack_frame.operand_stack.push(JavaValue::Integer(*x)),
            ValueConstant::Float(x) => stack_frame.operand_stack.push(JavaValue::Float(*x)),
            ValueConstant::Long(x) => stack_frame.operand_stack.push(JavaValue::Long(*x)),
            ValueConstant::Double(x) => stack_frame.operand_stack.push(JavaValue::Double(*x)),
            _ => unimplemented!(),
        }
    }
}
