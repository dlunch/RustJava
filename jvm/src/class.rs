use alloc::{boxed::Box, vec::Vec};

use crate::{ClassInstance, Field, JavaValue, JvmResult, Method};

pub trait Class {
    fn name(&self) -> &str;
    fn fields(&self) -> Vec<&dyn Field>;
    fn methods(&self) -> Vec<&dyn Method>;
    fn instantiate(&self) -> Box<dyn ClassInstance>;
    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue>;

    fn method(&self, name: &str, descriptor: &str) -> Option<&dyn Method> {
        self.methods()
            .into_iter()
            .find(|&method| method.name() == name && method.descriptor() == descriptor)
    }

    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<&dyn Field> {
        self.fields()
            .into_iter()
            .find(|&field| field.name() == name && field.descriptor() == descriptor && field.is_static() == is_static)
    }
}
