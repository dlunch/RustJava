use alloc::{boxed::Box, format, vec::Vec};
use core::iter;

use classfile::{AttributeInfoCode, Opcode, ValueConstant};

use jvm::{ClassInstance, JavaType, JavaValue, Jvm, JvmResult};

use crate::{stack_frame::StackFrame, thread::ThreadContextImpl};

pub struct Interpreter;

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
            tracing::trace!("Opcode {:?}", opcode);
            match opcode {
                Opcode::Aaload
                | Opcode::Baload
                | Opcode::Caload
                | Opcode::Daload
                | Opcode::Faload
                | Opcode::Iaload
                | Opcode::Laload
                | Opcode::Saload => {
                    // TODO type checking
                    let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let array = stack_frame.operand_stack.pop().unwrap();

                    let value = jvm.load_array(&array.into(), index as usize, 1).unwrap().pop().unwrap();

                    stack_frame.operand_stack.push(value);
                }
                Opcode::Aastore
                | Opcode::Bastore
                | Opcode::Castore
                | Opcode::Dastore
                | Opcode::Fastore
                | Opcode::Iastore
                | Opcode::Lastore
                | Opcode::Sastore => {
                    // TODO type checking
                    let value = stack_frame.operand_stack.pop().unwrap();
                    let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let mut array = stack_frame.operand_stack.pop().unwrap().into();

                    let element_type = jvm.array_element_type(&array).unwrap();

                    let value = if element_type == JavaType::Char || element_type == JavaType::Boolean {
                        Self::integer_to_type(value, element_type)
                    } else {
                        value
                    };

                    jvm.store_array(&mut array, index as usize, [value]).unwrap();
                }
                Opcode::AconstNull => {
                    stack_frame.operand_stack.push(JavaValue::Object(None));
                }
                Opcode::Aload(x) | Opcode::Iload(x) => {
                    let value = stack_frame.local_variables[*x as usize].clone();
                    stack_frame.operand_stack.push(value);
                }
                Opcode::Anewarray(x) => {
                    let length: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let element_type_name = format!("L{};", x.as_class());
                    let array = jvm.instantiate_array(&element_type_name, length as _).await?;

                    stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
                }
                Opcode::Areturn => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    return Ok(value);
                }
                Opcode::Arraylength => {
                    let array = stack_frame.operand_stack.pop().unwrap();

                    let length = jvm.array_length(&array.into())?;
                    stack_frame.operand_stack.push(JavaValue::Int(length as _));
                }
                Opcode::Astore(x) | Opcode::Istore(x) => {
                    let value = stack_frame.operand_stack.pop();
                    stack_frame.local_variables[*x as usize] = value.unwrap();
                }
                Opcode::Bipush(x) => {
                    stack_frame.operand_stack.push(JavaValue::Int(*x as i32));
                }
                Opcode::Dup => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                }
                Opcode::Getfield(x) => {
                    let instance = stack_frame.operand_stack.pop().unwrap();

                    let value = jvm.get_field(&instance.into(), &x.name, &x.descriptor)?;

                    stack_frame.operand_stack.push(value);
                }
                Opcode::Getstatic(x) => stack_frame
                    .operand_stack
                    .push(jvm.get_static_field(&x.class, &x.name, &x.descriptor).await?),
                Opcode::Goto(x) => {
                    iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                }
                Opcode::Iadd => {
                    let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                    stack_frame.operand_stack.push(JavaValue::Int(value1 + value2));
                }
                Opcode::Iconst(x) => {
                    stack_frame.operand_stack.push(JavaValue::Int(*x as i32));
                }
                Opcode::Invokevirtual(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();

                    let result = jvm.invoke_virtual(&instance.into(), &x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::Invokespecial(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let instance = stack_frame.operand_stack.pop().unwrap();

                    let result = jvm.invoke_special(&instance.into(), &x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::Invokestatic(x) => {
                    let params = Self::extract_invoke_params(&mut stack_frame, &x.descriptor);

                    let result = jvm.invoke_static(&x.class, &x.name, &x.descriptor, params).await?;
                    Self::push_invoke_result(&mut stack_frame, result);
                }
                Opcode::Idiv => {
                    let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                    stack_frame.operand_stack.push(JavaValue::Int(value1 / value2));
                }
                Opcode::IfAcmpeq(x) => {
                    let value2: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();

                    if value1.equals(&*value2)? {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfAcmpne(x) => {
                    let value2: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();

                    if !value1.equals(&*value2)? {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmpeq(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x == y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmpge(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x >= y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmpgt(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x > y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmple(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x <= y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmplt(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x < y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::IfIcmpne(x) => {
                    if Self::integer_condition(&mut stack_frame, |x, y| x != y) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifeq(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x == 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifge(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x >= 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifgt(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x > 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifle(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x <= 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Iflt(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x < 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifne(x) => {
                    if Self::integer_condition_single(&mut stack_frame, |x| x != 0) {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifnonnull(x) => {
                    let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                    if value.is_some() {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Ifnull(x) => {
                    let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                    if value.is_none() {
                        iter = code_attribute.code.range((*offset as i32 + *x as i32) as u32..);
                    }
                }
                Opcode::Iinc(x, y) => {
                    let value = stack_frame.local_variables[*x as usize].clone();
                    let value: i32 = value.into();

                    stack_frame.local_variables[*x as usize] = JavaValue::Int(value + *y as i32);
                }
                Opcode::Imul => {
                    let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                    stack_frame.operand_stack.push(JavaValue::Int(value1 * value2));
                }
                Opcode::Irem => {
                    let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                    stack_frame.operand_stack.push(JavaValue::Int(value1 % value2));
                }
                Opcode::Lconst(x) => {
                    stack_frame.operand_stack.push(JavaValue::Long(*x as i64));
                }
                Opcode::Ldc(x) | Opcode::LdcW(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
                Opcode::Ldc2W(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
                Opcode::New(x) => {
                    let class = jvm.instantiate_class(x.as_class()).await?;

                    stack_frame.operand_stack.push(JavaValue::Object(Some(class)));
                }
                Opcode::Newarray(x) => {
                    let element_type_name = match x {
                        4 => "Z",
                        5 => "C",
                        6 => "F",
                        7 => "D",
                        8 => "B",
                        9 => "S",
                        10 => "I",
                        11 => "J",
                        _ => panic!("Invalid array type {}", x),
                    };

                    let length: i32 = stack_frame.operand_stack.pop().unwrap().into();
                    let array = jvm.instantiate_array(element_type_name, length as _).await?;

                    stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
                }
                Opcode::Pop => {
                    stack_frame.operand_stack.pop().unwrap();
                }
                Opcode::Putfield(x) => {
                    let value = stack_frame.operand_stack.pop().unwrap();
                    let instance = stack_frame.operand_stack.pop().unwrap();

                    jvm.put_field(&mut instance.into(), &x.name, &x.descriptor, value)?;
                }
                Opcode::Putstatic(x) => {
                    jvm.put_static_field(&x.class, &x.name, &x.descriptor, stack_frame.operand_stack.pop().unwrap())
                        .await?;
                }
                Opcode::Return => {
                    return Ok(JavaValue::Void);
                }
                Opcode::Sipush(x) => {
                    stack_frame.operand_stack.push(JavaValue::Int(*x as i32));
                }
                _ => panic!("Unimplemented opcode {:?}", opcode),
            }
        }

        panic!("Should not reach here")
    }

    fn integer_condition<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32, i32) -> bool,
    {
        let value2 = Self::pop_integer(stack_frame);
        let value1 = Self::pop_integer(stack_frame);

        pred(value1, value2)
    }

    fn integer_condition_single<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32) -> bool,
    {
        let value = Self::pop_integer(stack_frame);

        pred(value)
    }

    fn pop_integer(stack_frame: &mut StackFrame) -> i32 {
        let value = stack_frame.operand_stack.pop().unwrap();

        match value {
            JavaValue::Boolean(x) => {
                if x {
                    1
                } else {
                    0
                }
            }
            JavaValue::Byte(x) => x as _,
            JavaValue::Char(x) => x as _,
            JavaValue::Short(x) => x as _,
            JavaValue::Int(x) => x,
            _ => panic!("Expected integer, got {:?}", value),
        }
    }

    fn integer_to_type(value: JavaValue, r#type: JavaType) -> JavaValue {
        let value: i32 = value.into();
        match r#type {
            JavaType::Boolean => JavaValue::Boolean(value != 0),
            JavaType::Byte => JavaValue::Byte(value as _),
            JavaType::Char => JavaValue::Char(value as _),
            JavaType::Short => JavaValue::Short(value as _),
            JavaType::Int => JavaValue::Int(value),
            _ => panic!("Expected integer type, got {:?}", r#type),
        }
    }

    fn extract_invoke_params(stack_frame: &mut StackFrame, descriptor: &str) -> Vec<JavaValue> {
        let method_type = JavaType::parse(descriptor);
        let (param_type, _) = method_type.as_method();

        let mut values = (0..param_type.len())
            .map(|_| stack_frame.operand_stack.pop().unwrap())
            .collect::<Vec<_>>();

        values.reverse();

        values
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
            ValueConstant::String(x) => JavaValue::Object(Some(Self::create_java_string(jvm, x).await?)),
            _ => unimplemented!(),
        })
    }

    async fn create_java_string(jvm: &mut Jvm, string: &str) -> JvmResult<Box<dyn ClassInstance>> {
        let chars = string.chars().map(|x| JavaValue::Char(x as _)).collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", chars.len()).await?;
        jvm.store_array(&mut array, 0, chars)?;

        let instance = jvm.instantiate_class("java/lang/String").await?;
        jvm.invoke_virtual(&instance, "java/lang/String", "<init>", "([C)V", [array.into()])
            .await?;

        Ok(instance)
    }
}
