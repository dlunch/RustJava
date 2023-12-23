use alloc::string::String;

use crate::{ArrayClassInstance, Field, JavaValue, JvmResult};

pub trait ClassInstance {
    fn class_name(&self) -> String;
    fn get_field(&self, field: &dyn Field) -> JvmResult<JavaValue>;
    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()>;
    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance>; // is there a better way to do this?
    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance>; // is there a better way to do this?
}
