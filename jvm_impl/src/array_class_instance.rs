use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use jvm::{ArrayClass, ArrayClassInstance, Class, ClassInstance, JavaType, JavaValue, JvmResult};

use crate::array_class::ArrayClassImpl;

pub struct ArrayClassInstanceImpl {
    class_name: String,
    length: usize,
    elements: Vec<JavaValue>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(&element_type).default();

        Self {
            class_name: class.name().to_string(),
            length,
            elements: vec![default_value; length],
        }
    }
}

impl ClassInstance for ArrayClassInstanceImpl {
    fn class_name(&self) -> String {
        self.class_name.clone()
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn jvm::Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn jvm::Field, _value: JavaValue) -> JvmResult<()> {
        panic!("Array classes do not have fields")
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn store(&mut self, offset: usize, values: &[JavaValue]) -> JvmResult<()> {
        anyhow::ensure!(offset + values.len() <= self.length, "Array index out of bounds");

        self.elements[offset..offset + values.len()].clone_from_slice(values);

        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> JvmResult<Vec<JavaValue>> {
        anyhow::ensure!(offset + length <= self.length, "Array index out of bounds");

        Ok(self.elements[offset..offset + length].to_vec())
    }

    fn length(&self) -> usize {
        self.length
    }
}
