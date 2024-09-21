use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};
use dyn_clone::clone_trait_object;

use crate::{class_definition::ClassDefinition, class_instance::ClassInstance, field::Field, method::Method, value::JavaValue, Result};

pub trait ArrayClassDefinition: ClassDefinition {
    fn element_type_name(&self) -> String;
    fn instantiate_array(&self, length: usize) -> Result<Box<dyn ClassInstance>>;
}

clone_trait_object!(ArrayClassDefinition);

#[async_trait::async_trait]
impl<T: ArrayClassDefinition> ClassDefinition for T {
    fn name(&self) -> String {
        format!("[{}", self.element_type_name())
    }

    fn super_class_name(&self) -> Option<String> {
        Some("java/lang/Object".to_string())
    }

    fn instantiate(&self) -> Result<Box<dyn ClassInstance>> {
        panic!("Cannot instantiate array class")
    }

    fn method(&self, _name: &str, _descriptor: &str, _is_static: bool) -> Option<Box<dyn Method>> {
        None
    }

    fn field(&self, _name: &str, _descriptor: &str, _is_static: bool) -> Option<Box<dyn Field>> {
        None
    }

    async fn get_static_field(&self, _field: &dyn Field) -> Result<JavaValue> {
        panic!("Array classes do not have static fields")
    }

    async fn put_static_field(&mut self, _field: &dyn Field, _value: JavaValue) -> Result<()> {
        panic!("Array classes do not have static fields")
    }

    fn as_array_class_definition(&self) -> Option<&dyn ArrayClassDefinition> {
        Some(self)
    }
}
