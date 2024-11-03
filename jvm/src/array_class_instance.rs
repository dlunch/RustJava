use alloc::{boxed::Box, vec::Vec};

use crate::{class_definition::ClassDefinition, class_instance::ClassInstance, field::Field, value::JavaValue, Result};

#[async_trait::async_trait]
pub trait ArrayClassInstance: ClassInstance {
    fn class_definition(&self) -> Box<dyn ClassDefinition>;
    fn destroy(self: Box<Self>);
    fn equals(&self, other: &dyn ClassInstance) -> Result<bool>;
    fn hash_code(&self) -> i32;
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()>;
    fn load(&self, offset: usize, count: usize) -> Result<Vec<JavaValue>>;
    fn raw_buffer(&self) -> Result<Box<dyn ArrayRawBuffer>>;
    fn raw_buffer_mut(&mut self) -> Result<Box<dyn ArrayRawBufferMut>>;
    fn length(&self) -> usize;
}

#[async_trait::async_trait]
impl<T: ArrayClassInstance> ClassInstance for T {
    fn destroy(self: Box<Self>) {
        ArrayClassInstance::destroy(self)
    }

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        ArrayClassInstance::class_definition(self)
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        ArrayClassInstance::equals(self, other)
    }

    fn hash_code(&self) -> i32 {
        ArrayClassInstance::hash_code(self)
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn Field) -> Result<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn Field, _value: JavaValue) -> Result<()> {
        panic!("Array classes do not have fields")
    }
}

pub trait ArrayRawBuffer {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;
}

pub trait ArrayRawBufferMut: ArrayRawBuffer {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()>;
}
