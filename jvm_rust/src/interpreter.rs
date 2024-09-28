use alloc::{boxed::Box, format, vec::Vec};
use core::iter;

use classfile::{AttributeInfoCode, Opcode, ValueConstant};
use jvm::{runtime::JavaLangString, ClassInstance, JavaChar, JavaError, JavaType, JavaValue, Jvm, Result};

use crate::stack_frame::StackFrame;

enum ExecuteNext {
    Continue,
    Jump(u32),
    Return(JavaValue),
}

pub struct Interpreter;

impl Interpreter {
    pub async fn run(jvm: &Jvm, code_attribute: &AttributeInfoCode, args: Box<[JavaValue]>, return_type: &JavaType) -> Result<JavaValue> {
        let mut stack_frame = StackFrame::new();

        stack_frame.local_variables = args.into_vec().into_iter().map(Self::to_stack_frame_type).collect();

        stack_frame
            .local_variables
            .extend(iter::repeat(JavaValue::Void).take(code_attribute.max_locals as usize));

        let mut iter = code_attribute.code.range(0..);
        while let Some((offset, opcode)) = iter.next() {
            tracing::trace!("Opcode {:?}", opcode);

            let result = Self::execute_opcode(jvm, *offset, opcode, &mut stack_frame, return_type).await;
            match result {
                Ok(ExecuteNext::Continue) => {}
                Ok(ExecuteNext::Jump(offset)) => {
                    iter = code_attribute.code.range(offset..);
                }
                Ok(ExecuteNext::Return(value)) => return Ok(value),
                Err(JavaError::JavaException(e)) => {
                    let exception_handler = Self::find_exception_handler(jvm, &*e, code_attribute, *offset).await;
                    if let Some(x) = exception_handler {
                        stack_frame.operand_stack.clear();
                        stack_frame.operand_stack.push(JavaValue::Object(Some(e)));

                        iter = code_attribute.code.range(x..);
                    } else {
                        return Err(JavaError::JavaException(e));
                    }
                }
                Err(e) => return Err(e),
            }
        }

        panic!("Should not reach here")
    }

    async fn execute_opcode(
        jvm: &Jvm,
        current_offset: u32,
        opcode: &Opcode,
        stack_frame: &mut StackFrame,
        return_type: &JavaType,
    ) -> Result<ExecuteNext> {
        match opcode {
            Opcode::Aaload | Opcode::Baload | Opcode::Caload | Opcode::Daload | Opcode::Faload | Opcode::Iaload | Opcode::Laload | Opcode::Saload => {
                // TODO type checking
                let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let array = stack_frame.operand_stack.pop().unwrap();

                let value = jvm.load_array(&array.into(), index as usize, 1).await?.pop().unwrap();

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
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

                let element_type = jvm.array_element_type(&array).await?;

                // operand stack has only integer, so convert it to the correct type
                let value = match element_type {
                    JavaType::Boolean => JavaValue::Boolean(i32::from(value) == 1),
                    JavaType::Byte => JavaValue::Byte(i32::from(value) as _),
                    JavaType::Char => JavaValue::Char(i32::from(value) as _),
                    JavaType::Short => JavaValue::Short(i32::from(value) as _),
                    JavaType::Int => JavaValue::Int(i32::from(value)),
                    _ => value,
                };

                jvm.store_array(&mut array, index as usize, [value]).await?;
            }
            Opcode::AconstNull => stack_frame.operand_stack.push(JavaValue::Object(None)),
            Opcode::Aload(x) | Opcode::Dload(x) | Opcode::Fload(x) | Opcode::Iload(x) | Opcode::Lload(x) => {
                let value = stack_frame.local_variables[*x as usize].clone();
                stack_frame.operand_stack.push(value);
            }
            Opcode::Athrow => {
                let exception = stack_frame.operand_stack.pop().unwrap().into();

                return Err(JavaError::JavaException(exception));
            }
            Opcode::Anewarray(x) => {
                let length: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let element_type_name = format!("L{};", x.as_class());
                let array = jvm.instantiate_array(&element_type_name, length as _).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
            Opcode::Areturn | Opcode::Dreturn | Opcode::Freturn | Opcode::Ireturn | Opcode::Lreturn => {
                let return_value = stack_frame.operand_stack.pop().unwrap();
                if matches!(opcode, Opcode::Ireturn) {
                    let value: i32 = return_value.into();
                    if *return_type == JavaType::Boolean {
                        return Ok(ExecuteNext::Return(JavaValue::Boolean(value == 1)));
                    } else if *return_type == JavaType::Char {
                        return Ok(ExecuteNext::Return(JavaValue::Char(value as _)));
                    } else if *return_type == JavaType::Byte {
                        return Ok(ExecuteNext::Return(JavaValue::Byte(value as _)));
                    } else if *return_type == JavaType::Short {
                        return Ok(ExecuteNext::Return(JavaValue::Short(value as _)));
                    } else {
                        return Ok(ExecuteNext::Return(JavaValue::Int(value)));
                    }
                }
                return Ok(ExecuteNext::Return(return_value));
            }
            Opcode::Arraylength => {
                let array = stack_frame.operand_stack.pop().unwrap();

                let length = jvm.array_length(&array.into()).await?;
                stack_frame.operand_stack.push(JavaValue::Int(length as _));
            }
            Opcode::Astore(x) | Opcode::Dstore(x) | Opcode::Fstore(x) | Opcode::Istore(x) | Opcode::Lstore(x) => {
                let value = stack_frame.operand_stack.pop();
                stack_frame.local_variables[*x as usize] = value.unwrap();
            }
            Opcode::Bipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Checkcast(_) => {
                todo!()
            }
            Opcode::D2f => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::D2i => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::D2l => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Dadd => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 + value2));
            }
            Opcode::Dcmpg => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Dcmpl => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(-1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Dconst(x) => {
                stack_frame.operand_stack.push(JavaValue::Double(*x as f64));
            }
            Opcode::Ddiv => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 / value2));
            }
            Opcode::Dmul => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 * value2));
            }
            Opcode::Dneg => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(-value));
            }
            Opcode::Dup => {
                let value = stack_frame.operand_stack.pop().unwrap();
                stack_frame.operand_stack.push(value.clone());
                stack_frame.operand_stack.push(value);
            }
            Opcode::Dup2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                } else {
                    let value2 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value);
                }
            }
            Opcode::Dup2X1 => {
                todo!()
            }
            Opcode::Dup2X2 => {
                todo!()
            }
            Opcode::DupX1 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1.clone());
                stack_frame.operand_stack.push(value2.clone());
                stack_frame.operand_stack.push(value1);
            }
            Opcode::DupX2 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();
                if matches!(value2, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3.clone());
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1);
                }
            }
            Opcode::Drem => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 % value2));
            }
            Opcode::Dsub => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 - value2));
            }
            Opcode::F2d => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::F2i => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::F2l => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Fadd => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 + value2));
            }
            Opcode::Fcmpg => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Fcmpl => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(-1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Fconst(x) => {
                stack_frame.operand_stack.push(JavaValue::Float(*x as f32));
            }
            Opcode::Fdiv => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 / value2));
            }
            Opcode::Fmul => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 * value2));
            }
            Opcode::Fneg => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(-value));
            }
            Opcode::Frem => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 % value2));
            }
            Opcode::Fsub => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 - value2));
            }
            Opcode::Getfield(x) => {
                let instance = stack_frame.operand_stack.pop().unwrap();

                let value = jvm.get_field(&instance.into(), &x.name, &x.descriptor).await?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Getstatic(x) => {
                let value = jvm.get_static_field(&x.class, &x.name, &x.descriptor).await?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Goto(x) => return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32)),
            Opcode::GotoW(x) => return Ok(ExecuteNext::Jump((current_offset as i32 + *x) as u32)),
            Opcode::I2b => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as u8 as _));
            }
            Opcode::I2c => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as JavaChar as _));
            }
            Opcode::I2d => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::I2f => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::I2l => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::I2s => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as u16 as _));
            }
            Opcode::Iadd => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 + value2));
            }
            Opcode::Iand => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 & value2));
            }
            Opcode::Iconst(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Idiv => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 / value2));
            }
            Opcode::IfAcmpeq(x) => {
                let value2: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();

                if value1.equals(&*value2)? {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfAcmpne(x) => {
                let value2: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();

                if !value1.equals(&*value2)? {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpeq(x) => {
                if Self::integer_condition(stack_frame, |x, y| x == y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpge(x) => {
                if Self::integer_condition(stack_frame, |x, y| x >= y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpgt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x > y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmple(x) => {
                if Self::integer_condition(stack_frame, |x, y| x <= y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmplt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x < y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpne(x) => {
                if Self::integer_condition(stack_frame, |x, y| x != y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifeq(x) => {
                if Self::integer_condition_single(stack_frame, |x| x == 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifge(x) => {
                if Self::integer_condition_single(stack_frame, |x| x >= 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifgt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x > 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifle(x) => {
                if Self::integer_condition_single(stack_frame, |x| x <= 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Iflt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x < 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifne(x) => {
                if Self::integer_condition_single(stack_frame, |x| x != 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnonnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_some() {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_none() {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
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
            Opcode::Ineg => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(-value));
            }
            Opcode::Instanceof(x) => {
                let instance: Box<dyn ClassInstance> = stack_frame.operand_stack.pop().unwrap().into();

                let result = jvm.is_instance(&*instance, x.as_class()).await?;
                stack_frame.operand_stack.push(JavaValue::Int(result as _));
            }
            Opcode::Invokedynamic(_) => {
                todo!()
            }
            Opcode::Invokeinterface(_, _, _) => {
                todo!()
            }
            Opcode::Invokespecial(x) => {
                let params = Self::extract_invoke_params(stack_frame, &x.descriptor);

                let instance = stack_frame.operand_stack.pop().unwrap();

                let result = jvm.invoke_special(&instance.into(), &x.class, &x.name, &x.descriptor, params).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokestatic(x) => {
                let params = Self::extract_invoke_params(stack_frame, &x.descriptor);

                let result = jvm.invoke_static(&x.class, &x.name, &x.descriptor, params).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokevirtual(x) => {
                let params = Self::extract_invoke_params(stack_frame, &x.descriptor);

                let instance = stack_frame.operand_stack.pop().unwrap();

                let result = jvm.invoke_virtual(&instance.into(), &x.name, &x.descriptor, params).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Ior => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 | value2));
            }
            Opcode::Irem => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 % value2));
            }
            Opcode::Ishl => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 << value2));
            }
            Opcode::Ishr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 >> value2));
            }
            Opcode::Isub => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 - value2));
            }
            Opcode::Iushr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(((value1 as u32) >> (value2 as u32)) as _));
            }
            Opcode::Ixor => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 ^ value2));
            }
            Opcode::Jsr(_) => {
                todo!()
            }
            Opcode::JsrW(_) => {
                todo!()
            }
            Opcode::L2d => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::L2f => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::L2i => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::Ladd => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 + value2));
            }
            Opcode::Land => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 & value2));
            }
            Opcode::Lcmp => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1.cmp(&value2) as _));
            }
            Opcode::Lconst(x) => stack_frame.operand_stack.push(JavaValue::Long(*x as i64)),
            Opcode::Ldc(x) | Opcode::LdcW(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
            Opcode::Ldc2W(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
            Opcode::Ldiv => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 / value2));
            }
            Opcode::Lmul => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 * value2));
            }
            Opcode::Lneg => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(-value));
            }
            Opcode::Lor => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 | value2));
            }
            Opcode::Lrem => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 % value2));
            }
            Opcode::Lshl => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 << value2));
            }
            Opcode::Lshr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 >> value2));
            }
            Opcode::Lsub => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 - value2));
            }
            Opcode::Lushr => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(((value1 as u64) >> (value2 as u64)) as _));
            }
            Opcode::Lxor => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 ^ value2));
            }
            Opcode::Lookupswitch(default, pairs) | Opcode::Tableswitch(default, pairs) => {
                let key = stack_frame.operand_stack.pop().unwrap().into();

                for (k, offset) in pairs {
                    if *k == key {
                        return Ok(ExecuteNext::Jump((current_offset as i32 + *offset) as u32));
                    }
                }

                return Ok(ExecuteNext::Jump((current_offset as i32 + *default) as u32));
            }
            Opcode::Monitorenter => {
                todo!()
            }
            Opcode::Monitorexit => {
                todo!()
            }
            Opcode::Multianewarray(x, d) => {
                let mut dimensions: Vec<i32> = (0..*d).map(|_| stack_frame.operand_stack.pop().unwrap().into()).collect();
                dimensions.reverse();

                let element_type_name = format!("L{};", x.as_class());
                let array = Self::new_multi_array(jvm, &element_type_name, &dimensions).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
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
            Opcode::Nop => {}
            Opcode::Pop => {
                stack_frame.operand_stack.pop().unwrap();
            }
            Opcode::Pop2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.pop().unwrap();
                }
            }
            Opcode::Putfield(x) => {
                let value = stack_frame.operand_stack.pop().unwrap();
                let instance = stack_frame.operand_stack.pop().unwrap();

                jvm.put_field(&mut instance.into(), &x.name, &x.descriptor, value).await?;
            }
            Opcode::Putstatic(x) => {
                jvm.put_static_field(&x.class, &x.name, &x.descriptor, stack_frame.operand_stack.pop().unwrap())
                    .await?
            }
            Opcode::Ret(_) => {
                todo!()
            }
            Opcode::Return => return Ok(ExecuteNext::Return(JavaValue::Void)),
            Opcode::Sipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Swap => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1);
                stack_frame.operand_stack.push(value2);
            }
            Opcode::Wide => {
                todo!()
            }
        }

        Ok(ExecuteNext::Continue)
    }

    async fn find_exception_handler(jvm: &Jvm, exception: &dyn ClassInstance, code_attribute: &AttributeInfoCode, pc: u32) -> Option<u32> {
        for exception_table in &code_attribute.exception_table {
            if exception_table.start_pc <= pc as u16 && exception_table.end_pc > pc as u16 {
                if exception_table.catch_type.is_none() {
                    return Some(exception_table.handler_pc as _);
                }

                let catch_type = exception_table.catch_type.as_ref().unwrap();
                if jvm.is_instance(exception, catch_type).await.unwrap() {
                    return Some(exception_table.handler_pc as _);
                }
            }
        }

        None
    }

    fn integer_condition<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32, i32) -> bool,
    {
        let value2 = stack_frame.operand_stack.pop().unwrap().into();
        let value1 = stack_frame.operand_stack.pop().unwrap().into();

        pred(value1, value2)
    }

    fn integer_condition_single<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32) -> bool,
    {
        let value = stack_frame.operand_stack.pop().unwrap().into();

        pred(value)
    }

    fn extract_invoke_params(stack_frame: &mut StackFrame, descriptor: &str) -> Vec<JavaValue> {
        let method_type = JavaType::parse(descriptor);
        let (param_type, _) = method_type.as_method();

        let mut values = param_type
            .iter()
            .rev()
            .map(|x| {
                let value = stack_frame.operand_stack.pop().unwrap();
                match x {
                    JavaType::Boolean => JavaValue::Boolean(i32::from(value) == 1),
                    JavaType::Byte => JavaValue::Byte(i32::from(value) as _),
                    JavaType::Char => JavaValue::Char(i32::from(value) as _),
                    JavaType::Short => JavaValue::Short(i32::from(value) as _),
                    _ => value,
                }
            })
            .collect::<Vec<_>>();

        values.reverse();

        values
    }

    fn push_invoke_result(stack_frame: &mut StackFrame, value: JavaValue) {
        match value {
            JavaValue::Void => {}
            _ => stack_frame.operand_stack.push(Self::to_stack_frame_type(value)),
        }
    }

    // convert into stack frame type, which is always integer for number types
    fn to_stack_frame_type(value: JavaValue) -> JavaValue {
        match value {
            JavaValue::Boolean(x) => JavaValue::Int(x as _),
            JavaValue::Byte(x) => JavaValue::Int(x as _),
            JavaValue::Char(x) => JavaValue::Int(x as _),
            JavaValue::Short(x) => JavaValue::Int(x as _),
            _ => value,
        }
    }

    async fn constant_to_value(jvm: &Jvm, constant: &ValueConstant) -> Result<JavaValue> {
        Ok(match constant {
            ValueConstant::Integer(x) => JavaValue::Int(*x),
            ValueConstant::Float(x) => JavaValue::Float(*x),
            ValueConstant::Long(x) => JavaValue::Long(*x),
            ValueConstant::Double(x) => JavaValue::Double(*x),
            ValueConstant::String(x) => JavaValue::Object(Some(JavaLangString::from_rust_string(jvm, x).await?)),
            ValueConstant::Class(x) => JavaValue::Object(Some(jvm.resolve_class(x).await?.java_class(jvm).await?)),
            _ => unimplemented!(),
        })
    }

    #[async_recursion::async_recursion]
    async fn new_multi_array(jvm: &Jvm, element_type_name: &str, dimensions: &[i32]) -> Result<Box<dyn ClassInstance>> {
        let element_type_name = "[".repeat(dimensions.len() - 1) + element_type_name;
        let mut array = jvm.instantiate_array(&element_type_name, dimensions[0] as _).await?;

        if dimensions.len() > 1 {
            for i in 0..dimensions[0] {
                let element = Self::new_multi_array(jvm, &element_type_name[1..], &dimensions[1..]).await?;
                jvm.store_array(&mut array, i as _, [element]).await?;
            }
        }

        Ok(array)
    }
}
