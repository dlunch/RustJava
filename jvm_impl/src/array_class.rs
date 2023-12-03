use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
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
    fn element_type_name(&self) -> &str {
        &self.element_type_name
    }

    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance> {
        Box::new(ArrayClassInstanceImpl::new(self, length))
    }
}

impl Class for ArrayClassImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn fields(&self) -> Vec<&dyn Field> {
        Vec::new()
    }

    fn methods(&self) -> Vec<&dyn Method> {
        Vec::new()
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        panic!("Cannot instantiate array class")
    }

    fn get_static_field(&self, _field: &dyn Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have static fields")
    }
}
