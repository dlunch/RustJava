use alloc::{boxed::Box, vec::Vec};
use core::iter;

use classfile::{AttributeInfoCode, Opcode, ValueConstant};

use jvm::{runtime::JavaLangString, JavaType, JavaValue, Jvm, JvmResult};

use crate::{stack_frame::StackFrame, thread::ThreadContextImpl};

pub struct Interpreter {}

impl Interpreter {
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn run(jvm: &mut Jvm, code_attribute: &AttributeInfoCode, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
        let thread_context = jvm.current_thread_context().as_any_mut().downcast_mut::<ThreadContextImpl>().unwrap();

        let stack_frame = thread_context.push_stack_frame();
        let mut stack_frame = stack_frame.borrow_mut();
        stack_frame.local_variables = args.into_vec();
        stack_frame
            .local_variables
            .extend(iter::repeat(JavaValue::Void).take(code_attribute.max_locals as usize));

        let mut iter = code_attribute.code.range(0..);
        while let Some((offset, opcode)) = iter.next() {
            match opcode {
                Opcode::Aaload => {
                    let index = stack_frame.operand_stack.pop().unwrap();
                    let array = stack_frame.operand_stack.pop().unwrap();

                    let array = array.as_object().unwrap();
                    let value = jvm.load_array(&array, index.as_int() as usize, 1).unwrap().pop().unwrap();

                    stack_frame.operand_stack.push(value);
                }
                Opcode::Aload(x) | Opcode::Iload(x) => {
                    let value = stack_frame.local_variables[*x as usize].clone();
                    stack_frame.operand_stack.push(value);
                }
                Opcode::Areturn => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    return Ok(value);
                }
                Opcode::Astore(x) | Opcode::Istore(x) => {
                    let value = stack_frame.operand_stack.pop();
                    stack_frame.local_variables[*x as usize] = value.unwrap();
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
                    iter = code_attribute.code.range(*x as u32..);
                }
                Opcode::Iconst(x) => {
                    stack_frame.operand_stack.push(JavaValue::Int(*x as i32));
                }
                Opcode::Invokevirtual(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();
                    let instance = instance.as_object().unwrap();

                    let result = jvm.invoke_virtual(&instance, &x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::Invokespecial(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();
                    let instance = instance.as_object().unwrap();

                    let result = jvm.invoke_special(&instance, &x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::Invokestatic(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let result = jvm.invoke_static(&x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::IfIcmpne(x) => {
                    let value2 = stack_frame.operand_stack.pop().unwrap();
                    let value1 = stack_frame.operand_stack.pop().unwrap();

                    if value1.as_int() != value2.as_int() {
                        iter = code_attribute.code.range(offset + *x as u32..);
                    }
                }
                Opcode::Irem => {
                    let value2 = stack_frame.operand_stack.pop().unwrap();
                    let value1 = stack_frame.operand_stack.pop().unwrap();

                    stack_frame.operand_stack.push(JavaValue::Int(value1.as_int() % value2.as_int()));
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

    fn push_invoke_result(stack_frame: &mut StackFrame, value: JavaValue) {
        match value {
            JavaValue::Void => {}
            _ => stack_frame.operand_stack.push(value),
        }
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
