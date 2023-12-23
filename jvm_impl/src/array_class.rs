use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};

use jvm::{ArrayClass, Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::array_class_instance::ArrayClassInstanceImpl;

pub struct ArrayClassImpl {
    name: String,
    element_type_name: String,
}

impl ArrayClassImpl {
    pub fn new(element_type_name: &str) -> Self {
        let name = format!("[{}", element_type_name);

        Self {
            name,
            element_type_name: element_type_name.to_string(),
        }
    }
}

impl ArrayClass for ArrayClassImpl {
    fn element_type_name(&self) -> String {
        self.element_type_name.clone()
    }

    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance> {
        Box::new(ArrayClassInstanceImpl::new(self, length))
    }
}

impl Class for ArrayClassImpl {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        panic!("Cannot instantiate array class")
    }

    fn method(&self, _name: &str, _descriptor: &str) -> Option<Box<dyn Method>> {
        None
    }

    fn field(&self, _name: &str, _descriptor: &str, _is_static: bool) -> Option<Box<dyn Field>> {
        None
    }

    fn get_static_field(&self, _field: &dyn Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have static fields")
    }

    fn put_static_field(&mut self, _field: &dyn Field, _value: JavaValue) -> JvmResult<()> {
        panic!("Array classes do not have static fields")
    }
}
