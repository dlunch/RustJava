use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};

use classfile::{Opcode, ValueConstant};

use jvm::{runtime::JavaLangString, JavaType, JavaValue, Jvm, JvmResult};

use crate::{stack_frame::StackFrame, thread::ThreadContextImpl};

pub struct Interpreter {}

impl Interpreter {
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn run(jvm: &mut Jvm, bytecode: &BTreeMap<u32, Opcode>, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
        let thread_context = jvm.current_thread_context().as_any_mut().downcast_mut::<ThreadContextImpl>().unwrap();

        let stack_frame = thread_context.push_stack_frame();
        let mut stack_frame = stack_frame.borrow_mut();
        stack_frame.local_variables = args.into_vec();

        let mut iter = bytecode.range(0..);
        while let Some((_, opcode)) = iter.next() {
            match opcode {
                Opcode::Aload0 => {
                    let value = stack_frame.local_variables[0].clone();
                    stack_frame.operand_stack.push(value);
                }
                Opcode::Astore1 => {
                    let value = stack_frame.operand_stack.pop();
                    stack_frame.local_variables[1] = value.unwrap();
                }
                Opcode::Dup => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                }
                Opcode::Getstatic(x) => stack_frame
                    .operand_stack
                    .push(jvm.get_static_field(&x.class, &x.name, &x.descriptor).await?),
                Opcode::Goto(x) => {
                    iter = bytecode.range(*x as u32..);
                }
                Opcode::Invokevirtual(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();
                    let instance = instance.as_object().unwrap();

                    jvm.invoke_method(&instance, &x.class, &x.name, &x.descriptor, params).await?;
                }
                Opcode::Invokespecial(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();
                    let instance = instance.as_object().unwrap();

                    jvm.invoke_special(&instance, &x.class, &x.name, &x.descriptor, params).await?;
                }
                Opcode::Ldc(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
                Opcode::New(x) => {
                    let class = jvm.instantiate_class(x.as_class()).await?;

                    stack_frame.operand_stack.push(JavaValue::Object(Some(class)));
                }
                Opcode::Return => {
                    return Ok(JavaValue::Void);
                }
                _ => panic!("Unimplemented opcode {:?}", opcode),
            }
        }

        panic!("Should not reach here")
    }

    fn extract_invoke_params(stack_frame: &mut StackFrame, descriptor: &str) -> Vec<JavaValue> {
        let method_type = JavaType::parse(descriptor);
        let (param_type, _) = method_type.as_method();

        (0..param_type.len())
            .map(|_| stack_frame.operand_stack.pop().unwrap())
            .collect::<Vec<_>>()
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
