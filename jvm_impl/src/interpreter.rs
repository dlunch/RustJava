use alloc::{collections::BTreeMap, vec::Vec};

use classfile::{Opcode, ValueConstant};

use jvm::{runtime::JavaLangString, JavaType, JavaValue, Jvm, JvmResult};

use crate::thread::ThreadContextImpl;

pub struct Interpreter {}

impl Interpreter {
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn run(jvm: &mut Jvm, bytecode: &BTreeMap<u32, Opcode>) -> JvmResult<JavaValue> {
        let thread_context = jvm.current_thread_context().as_any_mut().downcast_mut::<ThreadContextImpl>().unwrap();

        let stack_frame = thread_context.push_stack_frame();
        let mut stack_frame = stack_frame.borrow_mut();

        let mut iter = bytecode.range(0..);
        while let Some((_, opcode)) = iter.next() {
            match opcode {
                Opcode::Ldc(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
                Opcode::Getstatic(x) => stack_frame
                    .operand_stack
                    .push(jvm.get_static_field(&x.class, &x.name, &x.descriptor).await?),
                Opcode::Invokevirtual(x) => {
                    let method_type = JavaType::parse(&x.descriptor);
                    let (param_type, _) = method_type.as_method();

                    let params = (0..param_type.len())
                        .map(|_| stack_frame.operand_stack.pop().unwrap())
                        .collect::<Vec<_>>();

                    let instance = stack_frame.operand_stack.pop().unwrap();
                    let instance = instance.as_object().unwrap();

                    jvm.invoke_method(&instance, &x.class, &x.name, &x.descriptor, &params).await?;
                }
                Opcode::New(x) => {
                    let class = jvm.instantiate_class(x.as_class()).await?;

                    stack_frame.operand_stack.push(JavaValue::Object(Some(class)));
                }
                Opcode::Dup => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                }
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

    async fn constant_to_value(jvm: &mut Jvm, constant: &ValueConstant) -> JvmResult<JavaValue> {
        Ok(match constant {
            ValueConstant::Integer(x) => JavaValue::Int(*x),
            ValueConstant::Float(x) => JavaValue::Float(*x),
            ValueConstant::Long(x) => JavaValue::Long(*x),
            ValueConstant::Double(x) => JavaValue::Double(*x),
            ValueConstant::String(x) => JavaValue::Object(Some(JavaLangString::new(jvm, x).await?.instance)),
            _ => unimplemented!(),
        })
    }
}
