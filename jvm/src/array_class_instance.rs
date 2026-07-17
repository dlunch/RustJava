use alloc::{boxed::Box, vec::Vec};

use crate::{Result, class_instance::ClassInstance, value::JavaValue};

pub trait ArrayClassInstance: ClassInstance {
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()>;
    fn load(&self, offset: usize, count: usize) -> Result<Vec<JavaValue>>;
    fn raw_buffer(&self) -> Result<Box<dyn ArrayRawBuffer>>;
    fn raw_buffer_mut(&mut self) -> Result<Box<dyn ArrayRawBufferMut>>;
    fn length(&self) -> usize;
}

pub trait ArrayRawBuffer: Send {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;
}

pub trait ArrayRawBufferMut: ArrayRawBuffer {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()>;
}
