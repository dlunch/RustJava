use alloc::{boxed::Box, vec::Vec};

use crate::{class::Class, class_instance::ClassInstance, field::Field, value::JavaValue, JvmResult};

pub trait ArrayClassInstance: ClassInstance {
    fn class(&self) -> Box<dyn Class>;
    fn destroy(self: Box<Self>);
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> JvmResult<()>;
    fn load(&self, offset: usize, count: usize) -> JvmResult<Vec<JavaValue>>;
    fn store_bytes(&mut self, offset: usize, values: Box<[i8]>) -> JvmResult<()>;
    fn load_bytes(&self, offset: usize, count: usize) -> JvmResult<Vec<i8>>;
    fn length(&self) -> usize;
}

impl<T: ArrayClassInstance> ClassInstance for T {
    fn destroy(self: Box<Self>) {
        ArrayClassInstance::destroy(self)
    }

    fn class(&self) -> Box<dyn Class> {
        ArrayClassInstance::class(self)
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn Field, _value: JavaValue) -> JvmResult<()> {
        panic!("Array classes do not have fields")
    }
}
