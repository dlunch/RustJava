use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};
use dyn_clone::clone_trait_object;

use crate::{class_definition::ClassDefinition, class_instance::ClassInstance, field::Field, method::Method, value::JavaValue, JvmResult};

pub trait ArrayClass: ClassDefinition {
    fn element_type_name(&self) -> String;
    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance>;
}

clone_trait_object!(ArrayClass);

impl<T: ArrayClass> ClassDefinition for T {
    fn name(&self) -> String {
        format!("[{}", self.element_type_name())
    }

    fn super_class_name(&self) -> Option<String> {
        Some("java/lang/Object".to_string())
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

    fn as_array_class(&self) -> Option<&dyn ArrayClass> {
        Some(self)
    }
}
