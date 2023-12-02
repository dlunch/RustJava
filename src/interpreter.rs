use alloc::collections::BTreeMap;

use classfile::{Opcode, ValueConstant};

use crate::{runtime::JavaLangString, value::JavaValue, Jvm, JvmResult};

pub struct Interpreter {}

impl Interpreter {
    pub fn run(jvm: &mut Jvm, bytecode: &BTreeMap<u32, Opcode>) -> JvmResult<JavaValue> {
        let stack_frame = jvm.current_thread_context().current_stack_frame();
        let mut stack_frame = stack_frame.borrow_mut();

        let mut iter = bytecode.range(0..);
        while let Some((_, opcode)) = iter.next() {
            match opcode {
                Opcode::Ldc(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x)?),
                Opcode::Getstatic(x) => stack_frame.operand_stack.push(jvm.get_static_field(&x.class, &x.name)?),
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

    fn constant_to_value(jvm: &mut Jvm, constant: &ValueConstant) -> JvmResult<JavaValue> {
        Ok(match constant {
            ValueConstant::Integer(x) => JavaValue::Integer(*x),
            ValueConstant::Float(x) => JavaValue::Float(*x),
            ValueConstant::Long(x) => JavaValue::Long(*x),
            ValueConstant::Double(x) => JavaValue::Double(*x),
            ValueConstant::String(_) => JavaValue::ObjectRef(JavaLangString::new(jvm)?.instance),
            _ => unimplemented!(),
        })
    }
}
